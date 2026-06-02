#!/usr/bin/env python3
"""Render Dōbutsu Shōgi board positions and a piece-movement legend to SVG.

Original representation: generic shogi-style pentagon tiles + letters
(L Lion, G Giraffe, E Elephant, C Chick, H Hen). Not the official piece
artwork. Sente (player to move) tiles point up and sit on the bottom ranks;
Gote tiles point down on the top ranks.

Usage: python3 tools/render_board.py
Outputs to assets/diagrams/.
"""

import os

CELL = 66
MARGIN = 30
COLS = "ABC"
ROWS = [1, 2, 3, 4]

BOARD_FILL = "#f4ecd8"
BOARD_STROKE = "#8a7a55"
SENTE_FILL = "#fbf4df"
SENTE_STROKE = "#4a3f28"
SENTE_TEXT = "#2a2118"
GOTE_FILL = "#dde0e6"
GOTE_STROKE = "#3a3f48"
GOTE_TEXT = "#1f242b"
DOT = "#2b8a8a"
LABEL = "#7a6f55"

OUT = os.path.join(os.path.dirname(__file__), "..", "assets", "diagrams")


def pentagon(cx, cy, hw, hh, up=True):
    """Shogi-style 5-point tile centered at (cx,cy)."""
    if up:
        pts = [(cx, cy - hh), (cx + hw, cy - hh * 0.45),
               (cx + hw, cy + hh), (cx - hw, cy + hh), (cx - hw, cy - hh * 0.45)]
    else:
        pts = [(cx, cy + hh), (cx + hw, cy + hh * 0.45),
               (cx + hw, cy - hh), (cx - hw, cy - hh), (cx - hw, cy + hh * 0.45)]
    return " ".join(f"{x:.1f},{y:.1f}" for x, y in pts)


def tile(cx, cy, letter, owner, size=CELL * 0.40):
    fill, stroke, text = (SENTE_FILL, SENTE_STROKE, SENTE_TEXT) if owner == "sente" \
        else (GOTE_FILL, GOTE_STROKE, GOTE_TEXT)
    up = owner == "sente"
    fs = size * 1.05
    ty = cy + fs * 0.34
    return (
        f'<polygon points="{pentagon(cx, cy, size, size, up)}" '
        f'fill="{fill}" stroke="{stroke}" stroke-width="2" stroke-linejoin="round"/>'
        f'<text x="{cx:.1f}" y="{ty:.1f}" font-size="{fs:.1f}" font-weight="700" '
        f'text-anchor="middle" fill="{text}">{letter}</text>'
    )


def svg_open(w, h):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" '
            f'viewBox="0 0 {w} {h}" '
            f'font-family="-apple-system,Segoe UI,Helvetica,Arial,sans-serif">')


def render_position(pieces, outpath):
    """pieces: list of (file 'A'/'B'/'C', rank 1-4, letter, owner)."""
    w = MARGIN * 2 + 3 * CELL
    h = MARGIN * 2 + 4 * CELL
    s = [svg_open(w, h), f'<rect width="{w}" height="{h}" fill="white"/>']
    # squares
    for r in range(4):
        for c in range(3):
            x = MARGIN + c * CELL
            y = MARGIN + r * CELL
            s.append(f'<rect x="{x}" y="{y}" width="{CELL}" height="{CELL}" '
                     f'fill="{BOARD_FILL}" stroke="{BOARD_STROKE}" stroke-width="1.5"/>')
    # file/rank labels
    for c, f in enumerate(COLS):
        x = MARGIN + c * CELL + CELL / 2
        s.append(f'<text x="{x:.1f}" y="{MARGIN-10}" font-size="15" text-anchor="middle" '
                 f'fill="{LABEL}" font-weight="600">{f}</text>')
    for r, rk in enumerate(ROWS):
        y = MARGIN + r * CELL + CELL / 2 + 5
        s.append(f'<text x="{MARGIN-12}" y="{y:.1f}" font-size="15" text-anchor="middle" '
                 f'fill="{LABEL}" font-weight="600">{rk}</text>')
    # pieces
    for f, rk, letter, owner in pieces:
        c = COLS.index(f)
        r = ROWS.index(rk)
        cx = MARGIN + c * CELL + CELL / 2
        cy = MARGIN + r * CELL + CELL / 2
        s.append(tile(cx, cy, letter, owner))
    s.append("</svg>")
    write(outpath, "\n".join(s))


def render_moves(outpath):
    """Five 3x3 panels: how each piece moves (dots = reachable squares)."""
    panels = [
        ("Lion", "L", [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]),
        ("Giraffe", "G", [(0, -1), (0, 1), (-1, 0), (1, 0)]),
        ("Elephant", "E", [(-1, -1), (1, -1), (-1, 1), (1, 1)]),
        ("Chick", "C", [(0, -1)]),
        ("Hen", "H", [(0, -1), (-1, -1), (1, -1), (-1, 0), (1, 0), (0, 1)]),
    ]
    pc = 34          # panel cell size
    grid = pc * 3
    title_h = 26
    gap = 22
    pw = grid
    ph = grid + title_h
    w = len(panels) * pw + (len(panels) + 1) * gap
    h = ph + gap * 2
    s = [svg_open(w, h), f'<rect width="{w}" height="{h}" fill="white"/>']
    for i, (name, letter, targets) in enumerate(panels):
        ox = gap + i * (pw + gap)
        oy = gap + title_h
        s.append(f'<text x="{ox + grid/2:.1f}" y="{gap+16}" font-size="14" '
                 f'text-anchor="middle" fill="#333" font-weight="600">{name}</text>')
        for gr in range(3):
            for gc in range(3):
                x = ox + gc * pc
                y = oy + gr * pc
                s.append(f'<rect x="{x}" y="{y}" width="{pc}" height="{pc}" '
                         f'fill="{BOARD_FILL}" stroke="{BOARD_STROKE}" stroke-width="1"/>')
        # target dots (center is (1,1))
        for dx, dy in targets:
            cx = ox + (1 + dx) * pc + pc / 2
            cy = oy + (1 + dy) * pc + pc / 2
            s.append(f'<circle cx="{cx:.1f}" cy="{cy:.1f}" r="6" fill="{DOT}"/>')
        # the piece in the center
        cx = ox + 1 * pc + pc / 2
        cy = oy + 1 * pc + pc / 2
        s.append(tile(cx, cy, letter, "sente", size=pc * 0.38))
    s.append("</svg>")
    write(outpath, "\n".join(s))


def write(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as fh:
        fh.write(content + "\n")
    print("wrote", os.path.relpath(path))


INITIAL = [
    ("A", 1, "G", "gote"), ("B", 1, "L", "gote"), ("C", 1, "E", "gote"),
    ("B", 2, "C", "gote"),
    ("B", 3, "C", "sente"),
    ("A", 4, "E", "sente"), ("B", 4, "L", "sente"), ("C", 4, "G", "sente"),
]

if __name__ == "__main__":
    render_position(INITIAL, os.path.join(OUT, "initial-position.svg"))
    render_moves(os.path.join(OUT, "piece-moves.svg"))
