#!/usr/bin/env python3
"""Local server for the Dōbutsu Shōgi tablebase explorer.

Keeps one `probe` process alive (loads the tablebase once) and answers
/api?pos=<position-string> with that position's value and every legal move's
result. Serves the page and the Twemoji glyphs statically.

The probe is a single stdin/stdout pipe, so /api requests are serialized by a
lock, and the probe self-heals: if it dies or stops responding it is killed and
respawned and the request returns an error — rather than wedging the server,
which is exactly the failure you don't want during a launch traffic spike.

Run:  python3 explorer/serve.py   ->  http://localhost:41234/
Needs a probe binary + tablebase (see explorer/README.md).
"""
import http.server
import os
import re
import select
import socketserver
import subprocess
import threading
import urllib.parse

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
# Prefer our own tablebase; fall back to clausecker. All three speak the same
# stdin/stdout JSON protocol, so the rest is identical. The compact 333 MB
# build (ctbprobe + dobutsu.ctb) is preferred over the 2.14 GB records file.
COMPACT = (os.path.join(ROOT, "solver", "target", "release", "ctbprobe"),
           os.path.join(ROOT, "solver", "dobutsu.ctb"))
RECORDS = (os.path.join(ROOT, "solver", "target", "release", "tbprobe"),
           os.path.join(ROOT, "solver", "dobutsu.tb.bin"))
CLAUSECKER = (os.path.join(ROOT, "external", "clausecker-dobutsu", "probe"),
              os.path.join(ROOT, "external", "clausecker-dobutsu", "dobutsu.tb"))
if all(os.path.exists(p) for p in COMPACT):
    PROBE, TB = COMPACT
    print("backend: our own tablebase, compact (solver/dobutsu.ctb)")
elif all(os.path.exists(p) for p in RECORDS):
    PROBE, TB = RECORDS
    print("backend: our own tablebase, records (solver/dobutsu.tb.bin)")
else:
    PROBE, TB = CLAUSECKER
    print("backend: clausecker (our tablebase not built yet)")
EMOJI = os.path.join(ROOT, "assets", "diagrams", "emoji")
PORT = int(os.environ.get("PORT", 41234))  # Railway injects $PORT; default for local dev
INITIAL = "S/gle/-c-/-C-/ELG/-"

# Position strings are short and use only this alphabet. Reject anything else
# before it reaches the probe — the probe is the real validator, but this keeps
# pathological input out of the subprocess.
POS_RE = re.compile(r"\A[A-Za-z/-]{1,64}\Z")
# Seconds to wait for a reply before declaring the probe hung. Generous enough
# to cover a cold restart (reloading the 333 MB table), bounded enough that a
# genuinely stuck probe is detected and recycled.
PROBE_TIMEOUT = 20.0

_probe = None
_probe_lock = threading.Lock()


def _spawn_probe():
    return subprocess.Popen([PROBE, TB], stdin=subprocess.PIPE,
                            stdout=subprocess.PIPE, text=True, bufsize=1)


def _kill_probe():
    global _probe
    if _probe is not None:
        try:
            _probe.kill()
        except OSError:
            pass
    _probe = None


def query(pos):
    """Send one position to the probe and return its JSON reply line.

    Serialized (one shared pipe). Self-healing: a dead or hung probe is killed
    and respawned, and the request returns an error rather than blocking."""
    global _probe
    with _probe_lock:
        for _attempt in (1, 2):  # one retry across a respawn
            if _probe is None or _probe.poll() is not None:
                _probe = _spawn_probe()
            try:
                _probe.stdin.write(pos + "\n")
                _probe.stdin.flush()
            except (BrokenPipeError, OSError):
                _kill_probe()
                continue
            ready, _, _ = select.select([_probe.stdout], [], [], PROBE_TIMEOUT)
            if not ready:                 # hung — recycle and retry
                _kill_probe()
                continue
            line = _probe.stdout.readline()
            if line == "":                # EOF — probe died mid-reply
                _kill_probe()
                continue
            return line
        return '{"error":"tablebase probe unavailable"}\n'


class Handler(http.server.BaseHTTPRequestHandler):
    def _send(self, code, ctype, body):
        if not isinstance(body, bytes):
            body = body.encode()
        self.send_response(code)
        self.send_header("Content-Type", ctype)
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_GET(self):
        u = urllib.parse.urlparse(self.path)
        if u.path in ("/", "/index.html"):
            with open(os.path.join(HERE, "index.html"), "rb") as fh:
                self._send(200, "text/html; charset=utf-8", fh.read())
        elif u.path == "/api":
            q = urllib.parse.parse_qs(u.query)
            pos = q.get("pos", [INITIAL])[0]
            if not POS_RE.match(pos):
                self._send(400, "application/json", b'{"error":"invalid position"}')
            else:
                self._send(200, "application/json", query(pos))
        elif u.path.startswith("/emoji/"):
            name = os.path.basename(u.path)
            fp = os.path.join(EMOJI, name)
            if name.endswith(".svg") and os.path.isfile(fp):
                with open(fp, "rb") as fh:
                    self._send(200, "image/svg+xml", fh.read())
            else:
                self._send(404, "text/plain", b"not found")
        elif u.path == "/og.png":
            fp = os.path.join(HERE, "og.png")
            if os.path.isfile(fp):
                with open(fp, "rb") as fh:
                    self._send(200, "image/png", fh.read())
            else:
                self._send(404, "text/plain", b"not found")
        else:
            self._send(404, "text/plain", b"not found")

    def log_message(self, *a):
        pass


class Server(socketserver.ThreadingTCPServer):
    # Threaded so static assets (page, emoji, og) don't queue behind a slow /api
    # call; /api itself is serialized by _probe_lock around the single probe pipe.
    allow_reuse_address = True
    daemon_threads = True


if __name__ == "__main__":
    _probe = _spawn_probe()  # load the tablebase once at startup
    print(f"explorer: http://localhost:{PORT}/   (Ctrl-C to stop)")
    Server(("", PORT), Handler).serve_forever()
