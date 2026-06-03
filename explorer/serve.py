#!/usr/bin/env python3
"""Local server for the Dōbutsu Shōgi tablebase explorer.

Keeps one `probe` process alive (loads the tablebase once) and answers
/api?pos=<position-string> with that position's value and every legal move's
result. Serves the page and the Twemoji glyphs statically.

Run:  python3 explorer/serve.py   ->  http://localhost:41234/
Needs the probe binary built in external/clausecker-dobutsu (see explorer/README.md).
"""
import http.server
import os
import socketserver
import subprocess
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

probe = subprocess.Popen([PROBE, TB], stdin=subprocess.PIPE,
                         stdout=subprocess.PIPE, text=True, bufsize=1)


def query(pos):
    probe.stdin.write(pos + "\n")
    probe.stdin.flush()
    return probe.stdout.readline()


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
            self._send(200, "application/json", query(pos))
        elif u.path.startswith("/emoji/"):
            name = os.path.basename(u.path)
            fp = os.path.join(EMOJI, name)
            if name.endswith(".svg") and os.path.isfile(fp):
                with open(fp, "rb") as fh:
                    self._send(200, "image/svg+xml", fh.read())
            else:
                self._send(404, "text/plain", b"not found")
        else:
            self._send(404, "text/plain", b"not found")

    def log_message(self, *a):
        pass


class Server(socketserver.TCPServer):
    allow_reuse_address = True


if __name__ == "__main__":
    print(f"explorer: http://localhost:{PORT}/   (Ctrl-C to stop)")
    Server(("", PORT), Handler).serve_forever()
