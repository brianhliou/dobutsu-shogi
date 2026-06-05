#!/usr/bin/env python3
"""Resolve the perfect-game move list into [{move, pos}] for the explorer.

Walks data/perfect-game.txt through the compact tablebase probe, recording the
position that results from each move. The terminal move (lion capture) has
pos=null. Output is pasted into explorer/index.html as PERFECT_GAME so the
?game=perfect view loads instantly (no per-ply API calls in the browser).

    python3 tools/gen_perfect_game.py            # prints the JSON array
"""
import json
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PROBE = os.path.join(ROOT, "solver", "target", "release", "ctbprobe")
TB = os.path.join(ROOT, "solver", "dobutsu.ctb")
INITIAL = "S/gle/-c-/-C-/ELG/-"

moves = []
with open(os.path.join(ROOT, "data", "perfect-game.txt")) as fh:
    for line in fh:
        m = re.match(r"\s*\d+\.\s+(\S+)", line)
        if m:
            moves.append(m.group(1))

p = subprocess.Popen([PROBE, TB], stdin=subprocess.PIPE,
                     stdout=subprocess.PIPE, text=True, bufsize=1)


def query(pos):
    p.stdin.write(pos + "\n")
    p.stdin.flush()
    return json.loads(p.stdout.readline())


out = []
pos = INITIAL
for mv in moves:
    data = query(pos)
    entry = next((x for x in data["moves"] if x["move"] == mv), None)
    if entry is None:
        legal = [x["move"] for x in data["moves"]]
        print(f"MISMATCH: move {mv!r} not legal at {pos}. legal={legal}", file=sys.stderr)
        sys.exit(1)
    to = entry.get("to")  # resulting position, or null/absent for a game-ending move
    out.append({"move": mv, "pos": to})
    if not to:
        break
    pos = to

print(f"resolved {len(out)} plies; terminal pos={out[-1]['pos']!r}", file=sys.stderr)
print(json.dumps(out, ensure_ascii=False, separators=(",", ":")))
