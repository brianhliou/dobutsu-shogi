#!/usr/bin/env python3
"""Render Dōbutsu Shōgi board positions and a piece-movement legend to SVG.

Pieces are shogi-style pentagon tiles carrying an animal glyph. Owner is shown
by tile colour and pentagon orientation (first player's tiles point up and sit
warm; second player's point down and sit cool). Animal glyphs are Twemoji
(CC-BY 4.0), embedded via <defs>/<use>; see assets/diagrams/emoji/CREDITS.txt.

Usage: python3 tools/render_board.py   ->  assets/diagrams/*.svg
"""

import os
import re

CELL = 72
MARGIN = 32
COLS = "ABC"
ROWS = [1, 2, 3, 4]

BG = "#ffffff"
BOARD_FILL = "#efe2c0"
BOARD_LINE = "#c4b285"
BOARD_FRAME = "#9c8856"
SENTE_FILL = "#fcf7ea"
SENTE_STROKE = "#7a6a3f"
GOTE_FILL = "#e4e6ea"
GOTE_STROKE = "#6a6f78"
DOT = "#2b8a8a"
LABEL = "#9a8a62"

PIECE = {"L": "lion", "G": "giraffe", "E": "elephant", "C": "chick", "H": "hen"}

HERE = os.path.dirname(__file__)
OUT = os.path.join(HERE, "..", "assets", "diagrams")
EMOJI_DIR = os.path.join(OUT, "emoji")

_emoji_cache = {}


def emoji(name):
    if name not in _emoji_cache:
        txt = open(os.path.join(EMOJI_DIR, f"{name}.svg")).read()
        vb = re.search(r'viewBox="([^"]+)"', txt).group(1)
        inner = txt[txt.index(">", txt.index("<svg")) + 1: txt.rindex("</svg>")]
        _emoji_cache[name] = (vb, inner)
    return _emoji_cache[name]


def defs(names):
    syms = []
    for n in sorted(set(names)):
        vb, inner = emoji(n)
        syms.append(f'<symbol id="e_{n}" viewBox="{vb}">{inner}</symbol>')
    return "<defs>" + "".join(syms) + "</defs>"


def pentagon(cx, cy, h, up=True):
    if up:
        pts = [(cx, cy - h), (cx + h, cy - h * 0.45),
               (cx + h, cy + h), (cx - h, cy + h), (cx - h, cy - h * 0.45)]
    else:
        pts = [(cx, cy + h), (cx + h, cy + h * 0.45),
               (cx + h, cy - h), (cx - h, cy - h), (cx - h, cy + h * 0.45)]
    return " ".join(f"{x:.1f},{y:.1f}" for x, y in pts)


def tile(cx, cy, letter, owner, half=CELL * 0.42, glyph=CELL * 0.52):
    fill, stroke = (SENTE_FILL, SENTE_STROKE) if owner == "sente" else (GOTE_FILL, GOTE_STROKE)
    up = owner == "sente"
    name = PIECE[letter]
    d = glyph
    return (
        f'<polygon points="{pentagon(cx, cy, half, up)}" fill="{fill}" '
        f'stroke="{stroke}" stroke-width="2" stroke-linejoin="round"/>'
        f'<use href="#e_{name}" x="{cx-d/2:.1f}" y="{cy-d/2:.1f}" width="{d:.1f}" height="{d:.1f}"/>'
    )


def svg_open(w, h):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" '
            f'xmlns:xlink="http://www.w3.org/1999/xlink" width="{w}" height="{h}" '
            f'viewBox="0 0 {w} {h}" '
            f'font-family="-apple-system,Segoe UI,Helvetica,Arial,sans-serif">')


def render_position(pieces, outpath):
    w = MARGIN * 2 + 3 * CELL
    h = MARGIN * 2 + 4 * CELL
    s = [svg_open(w, h), f'<rect width="{w}" height="{h}" fill="{BG}"/>']
    s.append(defs(PIECE[p[2]] for p in pieces))
    s.append(f'<rect x="{MARGIN-3}" y="{MARGIN-3}" width="{3*CELL+6}" height="{4*CELL+6}" '
             f'fill="none" stroke="{BOARD_FRAME}" stroke-width="3" rx="4"/>')
    for r in range(4):
        for c in range(3):
            x, y = MARGIN + c * CELL, MARGIN + r * CELL
            s.append(f'<rect x="{x}" y="{y}" width="{CELL}" height="{CELL}" '
                     f'fill="{BOARD_FILL}" stroke="{BOARD_LINE}" stroke-width="1"/>')
    for c, f in enumerate(COLS):
        x = MARGIN + c * CELL + CELL / 2
        s.append(f'<text x="{x:.1f}" y="{MARGIN-10}" font-size="15" text-anchor="middle" '
                 f'fill="{LABEL}" font-weight="600">{f}</text>')
    for r, rk in enumerate(ROWS):
        y = MARGIN + r * CELL + CELL / 2 + 5
        s.append(f'<text x="{MARGIN-13}" y="{y:.1f}" font-size="15" text-anchor="middle" '
                 f'fill="{LABEL}" font-weight="600">{rk}</text>')
    for f, rk, letter, owner in pieces:
        cx = MARGIN + COLS.index(f) * CELL + CELL / 2
        cy = MARGIN + ROWS.index(rk) * CELL + CELL / 2
        s.append(tile(cx, cy, letter, owner))
    s.append("</svg>")
    write(outpath, "\n".join(s))


def render_moves(outpath):
    panels = [
        ("Lion", "L", [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]),
        ("Giraffe", "G", [(0, -1), (0, 1), (-1, 0), (1, 0)]),
        ("Elephant", "E", [(-1, -1), (1, -1), (-1, 1), (1, 1)]),
        ("Chick", "C", [(0, -1)]),
        ("Hen", "H", [(0, -1), (-1, -1), (1, -1), (-1, 0), (1, 0), (0, 1)]),
    ]
    pc, title_h, gap = 38, 26, 24
    grid = pc * 3
    pw, ph = grid, grid + title_h
    w = len(panels) * pw + (len(panels) + 1) * gap
    h = ph + gap * 2
    s = [svg_open(w, h), f'<rect width="{w}" height="{h}" fill="{BG}"/>']
    s.append(defs(PIECE[p[1]] for p in panels))
    for i, (name, letter, targets) in enumerate(panels):
        ox = gap + i * (pw + gap)
        oy = gap + title_h
        s.append(f'<text x="{ox+grid/2:.1f}" y="{gap+15}" font-size="14" '
                 f'text-anchor="middle" fill="#333" font-weight="600">{name}</text>')
        for gr in range(3):
            for gc in range(3):
                s.append(f'<rect x="{ox+gc*pc}" y="{oy+gr*pc}" width="{pc}" height="{pc}" '
                         f'fill="{BOARD_FILL}" stroke="{BOARD_LINE}" stroke-width="1"/>')
        for dx, dy in targets:
            cx = ox + (1 + dx) * pc + pc / 2
            cy = oy + (1 + dy) * pc + pc / 2
            s.append(f'<circle cx="{cx:.1f}" cy="{cy:.1f}" r="6" fill="{DOT}"/>')
        cx, cy = ox + pc + pc / 2, oy + pc + pc / 2
        s.append(tile(cx, cy, letter, "sente", half=pc * 0.42, glyph=pc * 0.62))
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
