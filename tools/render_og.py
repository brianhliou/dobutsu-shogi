#!/usr/bin/env python3
"""Capture the Open Graph / README share image for the tablebase explorer.

The card *is* a real screenshot of the explorer — the frame at ply 7 of the
perfect game, where the panel shows the single winning move (Lb1xa2, mate in 71)
ranked above the cliff of losing moves. That one-move-holds-the-win view is the
clearest visual proof of what a complete tablebase is.

Reproduce (capture reflects whatever code serve.py is running, so it stays in
sync with the explorer instead of drifting like a hand-built card):

    python3 explorer/serve.py &            # serve the current explorer + tablebase
    python3 tools/render_og.py             # -> explorer/og.png (1200x630)

Pipeline: headless Chrome shoots the ?game=perfect&ply=7 deep-link at 2x; the
tall three-column app screenshot is then scaled to fit and white-padded to the
2400x1260 OG aspect (1.905:1, == 1200x630). White == the page background, so the
margins are seamless and also vanish into GitHub's README background. The image
ships at 2x: social cards downsample it, while the README (rendered ~720px wide
on retina) stays crisp instead of upscaling a 1x file.

Deps (macOS): Google Chrome and `sips` (built in). Overrides:
    OG_URL=https://dobutsu.brianhliou.com/?game=perfect&ply=7   # capture live instead
    CHROME=/path/to/chrome
"""
import os
import subprocess
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
OUT = os.path.join(ROOT, "explorer", "og.png")

URL = os.environ.get("OG_URL", "http://localhost:41234/?game=perfect&ply=7")
CHROME = os.environ.get(
    "CHROME", "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")


def run(*cmd):
    subprocess.run(cmd, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)


def main():
    with tempfile.TemporaryDirectory() as tmp:
        shot = os.path.join(tmp, "shot.png")          # 2400x1584 (1200x792 @2x)
        run(CHROME, "--headless", "--disable-gpu", "--hide-scrollbars",
            "--force-device-scale-factor=2", "--window-size=1200,792",
            "--virtual-time-budget=10000", f"--screenshot={shot}", URL)
        scaled = os.path.join(tmp, "scaled.png")
        run("sips", "--resampleHeight", "1260", shot, "--out", scaled)        # fit OG height
        run("sips", "--padToHeightWidth", "1260", "2400", "--padColor", "FFFFFF",
            scaled, "--out", OUT)                                             # 2400x1260, white (2x)
    print("wrote", os.path.relpath(OUT, ROOT))


if __name__ == "__main__":
    main()
