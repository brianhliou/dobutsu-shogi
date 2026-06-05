#!/usr/bin/env python3
"""Render the 1200x630 Open Graph share image (explorer/og-card.svg).

Reuses render_board's emoji + tile helpers to draw the initial board on a
branded card, then convert to PNG:

    python3 tools/render_og.py
    rsvg-convert explorer/og-card.svg -o explorer/og.png
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import render_board as rb  # noqa: E402

W, H = 1200, 630
CELL = 116
COLS, ROWS = "ABC", [1, 2, 3, 4]
BW, BH = 3 * CELL, 4 * CELL
BX, BY = 96, (H - BH) // 2

INITIAL = [
    ("A", 1, "G", "gote"), ("B", 1, "L", "gote"), ("C", 1, "E", "gote"),
    ("B", 2, "C", "gote"),
    ("B", 3, "C", "sente"),
    ("A", 4, "E", "sente"), ("B", 4, "L", "sente"), ("C", 4, "G", "sente"),
]


def main():
    names = [rb.PIECE[p[2]] for p in INITIAL]
    s = [f'<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" '
         f'width="{W}" height="{H}" viewBox="0 0 {W} {H}" '
         f'font-family="-apple-system,Segoe UI,Helvetica,Arial,sans-serif">']
    s.append(f'<rect width="{W}" height="{H}" fill="#fbf7ee"/>')
    s.append(rb.defs(names))

    # board card + field zones
    s.append(f'<rect x="{BX-12}" y="{BY-12}" width="{BW+24}" height="{BH+24}" rx="18" '
             f'fill="#ffffff" stroke="#e7dcc0" stroke-width="2"/>')
    s.append(f'<rect x="{BX}" y="{BY}" width="{BW}" height="{CELL}" fill="#d9ecf6"/>')
    s.append(f'<rect x="{BX}" y="{BY+CELL}" width="{BW}" height="{2*CELL}" fill="#f1e6c6"/>')
    s.append(f'<rect x="{BX}" y="{BY+3*CELL}" width="{BW}" height="{CELL}" fill="#cfe3aa"/>')
    for i in range(1, 3):
        s.append(f'<line x1="{BX+i*CELL}" y1="{BY}" x2="{BX+i*CELL}" y2="{BY+BH}" stroke="#00000016"/>')
    for i in range(1, 4):
        s.append(f'<line x1="{BX}" y1="{BY+i*CELL}" x2="{BX+BW}" y2="{BY+i*CELL}" stroke="#00000016"/>')
    s.append(f'<rect x="{BX}" y="{BY}" width="{BW}" height="{BH}" fill="none" stroke="#9c8856" stroke-width="3"/>')
    for f, rk, letter, owner in INITIAL:
        cx = BX + COLS.index(f) * CELL + CELL / 2
        cy = BY + ROWS.index(rk) * CELL + CELL / 2
        s.append(rb.tile(cx, cy, letter, owner, half=CELL * 0.40, glyph=CELL * 0.50))

    # title block on the right
    tx = BX + BW + 76
    s.append(f'<text x="{tx}" y="214" font-size="66" font-weight="800" fill="#1f2328">Dōbutsu Shōgi,</text>')
    s.append(f'<text x="{tx}" y="288" font-size="66" font-weight="800" fill="#d98a2b">solved.</text>')
    s.append(f'<text x="{tx}" y="356" font-size="30" fill="#444">The 3×4 children’s shogi, played</text>')
    s.append(f'<text x="{tx}" y="396" font-size="30" fill="#444">out to the last move.</text>')
    s.append(f'<text x="{tx}" y="470" font-size="27" font-weight="700" fill="#2e7d32">Second player wins in 78 plies.</text>')
    s.append(f'<text x="{tx}" y="548" font-size="23" fill="#7a6f5a">246,803,167 positions · brianhliou.com</text>')

    s.append('</svg>')
    out = os.path.join(os.path.dirname(__file__), "..", "explorer", "og-card.svg")
    with open(out, "w") as fh:
        fh.write("\n".join(s) + "\n")
    print("wrote", os.path.relpath(out))


if __name__ == "__main__":
    main()
