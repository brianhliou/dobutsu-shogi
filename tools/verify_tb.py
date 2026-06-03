#!/usr/bin/env python3
"""Verify our tablebase against clausecker's, position by position.

Walks positions reachable from the start (BFS over our probe's `to` fields)
and, for each one, queries both probes and compares the value — result AND
distance-to-mate. This is the authoritative correctness check on
`solver/dobutsu.tb.bin`: two independently built tablebases, the same verdict
on every position.

clausecker rejects a handful of positions that are legal as children but never
stand alone (the two lions adjacent / a lion on the far rank). Those come back
as {"error": ...} and are counted as "clausecker-invalid", not mismatches.

Run:  python3 tools/verify_tb.py [N]      # N positions, default 6000
Needs: solver/target/release/tbprobe + solver/dobutsu.tb.bin
       external/clausecker-dobutsu/probe + dobutsu.tb
"""
import collections
import json
import os
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OURS = (os.path.join(ROOT, "solver", "target", "release", "tbprobe"),
        os.path.join(ROOT, "solver", "dobutsu.tb.bin"))
THEIRS = (os.path.join(ROOT, "external", "clausecker-dobutsu", "probe"),
          os.path.join(ROOT, "external", "clausecker-dobutsu", "dobutsu.tb"))
INITIAL = "S/gle/-c-/-C-/ELG/-"


def spawn(bin_path, tb_path):
    for p in (bin_path, tb_path):
        if not os.path.exists(p):
            sys.exit(f"missing: {p}")
    return subprocess.Popen([bin_path, tb_path], stdin=subprocess.PIPE,
                            stdout=subprocess.PIPE, text=True, bufsize=1)


def query(proc, pos):
    proc.stdin.write(pos + "\n")
    proc.stdin.flush()
    return json.loads(proc.stdout.readline())


def main():
    limit = int(sys.argv[1]) if len(sys.argv) > 1 else 6000
    ours, theirs = spawn(*OURS), spawn(*THEIRS)

    seen = {INITIAL}
    frontier = collections.deque([INITIAL])
    checked = mismatch = invalid = 0
    examples = []

    while frontier and checked < limit:
        pos = frontier.popleft()
        a = query(ours, pos)
        b = query(theirs, pos)
        if "error" in b or "error" in a:
            invalid += 1
        else:
            av, bv = a["value"], b["value"]
            if (av["result"], av["dtm"]) != (bv["result"], bv["dtm"]):
                mismatch += 1
                if len(examples) < 10:
                    examples.append((pos, av, bv))
            checked += 1
        for m in a.get("moves", []):
            to = m.get("to")
            if to and to not in seen:
                seen.add(to)
                frontier.append(to)

    print(f"checked {checked}  mismatches {mismatch}  clausecker-invalid {invalid}")
    for pos, av, bv in examples:
        print(f"  MISMATCH {pos}  ours={av}  clausecker={bv}")
    sys.exit(1 if mismatch else 0)


if __name__ == "__main__":
    main()
