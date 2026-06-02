#!/usr/bin/env python3
"""Render Dōbutsu Shōgi positions and a piece-movement legend to SVG.

Pieces are shogi-style pentagon tiles carrying an animal glyph. Owner is shown
by tile colour and pentagon orientation (first player's tiles point up and sit
warm; second player's point down and sit cool). Animal glyphs are Twemoji
(CC-BY 4.0), embedded via <defs>/<use>; see assets/diagrams/emoji/CREDITS.txt.

Positions can be given as the clausecker/dobutsu position string, e.g.
"S/cgl/--e/--L/c-G/E" (side / rank1 / rank2 / rank3 / rank4 / hand).

Usage: python3 tools/render_board.py   ->  assets/diagrams/*.svg
"""

import os
import re

CELL = 72
MARGIN = 32
GAP = 18
HAND_W = CELL
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
HILITE = "#d98a2b"
LABEL = "#9a8a62"

PIECE = {"L": "lion", "G": "giraffe", "E": "elephant", "C": "chick", "H": "hen"}
CHAR = {"C": ("C", "sente"), "c": ("C", "gote"), "G": ("G", "sente"), "g": ("G", "gote"),
        "E": ("E", "sente"), "e": ("E", "gote"), "L": ("L", "sente"), "l": ("L", "gote"),
        "R": ("H", "sente"), "r": ("H", "gote")}

HERE = os.path.dirname(__file__)
OUT = os.path.join(HERE, "..", "assets", "diagrams")
EMOJI_DIR = os.path.join(OUT, "emoji")
_emoji = {}


def emoji(name):
    if name not in _emoji:
        txt = open(os.path.join(EMOJI_DIR, f"{name}.svg")).read()
        vb = re.search(r'viewBox="([^"]+)"', txt).group(1)
        inner = txt[txt.index(">", txt.index("<svg")) + 1: txt.rindex("</svg>")]
        _emoji[name] = (vb, inner)
    return _emoji[name]


def defs(names):
    syms = [f'<symbol id="e_{n}" viewBox="{emoji(n)[0]}">{emoji(n)[1]}</symbol>'
            for n in sorted(set(names))]
    return "<defs>" + "".join(syms) + "</defs>"


def pentagon(cx, cy, h, up=True):
    if up:
        pts = [(cx, cy - h), (cx + h, cy - h * 0.45), (cx + h, cy + h),
               (cx - h, cy + h), (cx - h, cy - h * 0.45)]
    else:
        pts = [(cx, cy + h), (cx + h, cy + h * 0.45), (cx + h, cy - h),
               (cx - h, cy - h), (cx - h, cy + h * 0.45)]
    return " ".join(f"{x:.1f},{y:.1f}" for x, y in pts)


def tile(cx, cy, letter, owner, half=CELL * 0.42, glyph=CELL * 0.52):
    fill, stroke = (SENTE_FILL, SENTE_STROKE) if owner == "sente" else (GOTE_FILL, GOTE_STROKE)
    name = PIECE[letter]
    return (f'<polygon points="{pentagon(cx, cy, half, owner == "sente")}" fill="{fill}" '
            f'stroke="{stroke}" stroke-width="2" stroke-linejoin="round"/>'
            f'<use href="#e_{name}" x="{cx-glyph/2:.1f}" y="{cy-glyph/2:.1f}" '
            f'width="{glyph:.1f}" height="{glyph:.1f}"/>')


def svg_open(w, h):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" '
            f'xmlns:xlink="http://www.w3.org/1999/xlink" width="{w}" height="{h}" '
            f'viewBox="0 0 {w} {h}" '
            f'font-family="-apple-system,Segoe UI,Helvetica,Arial,sans-serif">')


def render_position(pieces, outpath, sente_hand=(), gote_hand=(), highlight=None):
    has_hand = bool(sente_hand or gote_hand)
    board_right = MARGIN + 3 * CELL
    w = board_right + (GAP + HAND_W + MARGIN if has_hand else MARGIN)
    h = MARGIN * 2 + 4 * CELL
    names = [PIECE[p[2]] for p in pieces] + [PIECE[x] for x in (*sente_hand, *gote_hand)]
    s = [svg_open(w, h), f'<rect width="{w}" height="{h}" fill="{BG}"/>', defs(names)]
    s.append(f'<rect x="{MARGIN-3}" y="{MARGIN-3}" width="{3*CELL+6}" height="{4*CELL+6}" '
             f'fill="none" stroke="{BOARD_FRAME}" stroke-width="3" rx="4"/>')
    for r in range(4):
        for c in range(3):
            s.append(f'<rect x="{MARGIN+c*CELL}" y="{MARGIN+r*CELL}" width="{CELL}" '
                     f'height="{CELL}" fill="{BOARD_FILL}" stroke="{BOARD_LINE}" stroke-width="1"/>')
    for c, f in enumerate(COLS):
        s.append(f'<text x="{MARGIN+c*CELL+CELL/2:.1f}" y="{MARGIN-10}" font-size="15" '
                 f'text-anchor="middle" fill="{LABEL}" font-weight="600">{f}</text>')
    for r, rk in enumerate(ROWS):
        s.append(f'<text x="{MARGIN-13}" y="{MARGIN+r*CELL+CELL/2+5:.1f}" font-size="15" '
                 f'text-anchor="middle" fill="{LABEL}" font-weight="600">{rk}</text>')
    if highlight:
        hc = MARGIN + COLS.index(highlight[0]) * CELL + CELL / 2
        hr = MARGIN + (highlight[1] - 1) * CELL + CELL / 2
        s.append(f'<circle cx="{hc:.1f}" cy="{hr:.1f}" r="{CELL*0.40:.1f}" fill="{HILITE}" '
                 f'fill-opacity="0.16" stroke="{HILITE}" stroke-width="3.5"/>')
    for f, rk, letter, owner in pieces:
        cx = MARGIN + COLS.index(f) * CELL + CELL / 2
        cy = MARGIN + ROWS.index(rk) * CELL + CELL / 2
        s.append(tile(cx, cy, letter, owner))
    if has_hand:
        hx = board_right + GAP + HAND_W / 2
        hh, hg, step = CELL * 0.30, CELL * 0.42, CELL * 0.76
        if gote_hand:
            s.append(f'<text x="{hx:.1f}" y="{MARGIN+12}" font-size="11" text-anchor="middle" '
                     f'fill="{LABEL}">in hand</text>')
            for i, x in enumerate(gote_hand):
                s.append(tile(hx, MARGIN + hh + 22 + i * step, x, "gote", half=hh, glyph=hg))
        if sente_hand:
            base = MARGIN + 4 * CELL
            s.append(f'<text x="{hx:.1f}" y="{base-len(sente_hand)*step-hh-8:.1f}" font-size="11" '
                     f'text-anchor="middle" fill="{LABEL}">in hand</text>')
            for i, x in enumerate(sente_hand):
                s.append(tile(hx, base - hh - i * step, x, "sente", half=hh, glyph=hg))
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
    s = [svg_open(w, h), f'<rect width="{w}" height="{h}" fill="{BG}"/>',
         defs(PIECE[p[1]] for p in panels)]
    for i, (name, letter, targets) in enumerate(panels):
        ox, oy = gap + i * (pw + gap), gap + title_h
        s.append(f'<text x="{ox+grid/2:.1f}" y="{gap+15}" font-size="14" '
                 f'text-anchor="middle" fill="#333" font-weight="600">{name}</text>')
        for gr in range(3):
            for gc in range(3):
                s.append(f'<rect x="{ox+gc*pc}" y="{oy+gr*pc}" width="{pc}" height="{pc}" '
                         f'fill="{BOARD_FILL}" stroke="{BOARD_LINE}" stroke-width="1"/>')
        for dx, dy in targets:
            s.append(f'<circle cx="{ox+(1+dx)*pc+pc/2:.1f}" cy="{oy+(1+dy)*pc+pc/2:.1f}" '
                     f'r="6" fill="{DOT}"/>')
        s.append(tile(ox + pc + pc / 2, oy + pc + pc / 2, letter, "sente", half=pc * 0.42, glyph=pc * 0.62))
    s.append("</svg>")
    write(outpath, "\n".join(s))


def parse_posstring(code):
    parts = code.split("/")
    pieces = []
    for r in range(4):
        for c in range(3):
            ch = parts[1 + r][c]
            if ch != "-":
                letter, owner = CHAR[ch]
                pieces.append((COLS[c], r + 1, letter, owner))
    sente_hand, gote_hand = [], []
    hand = parts[5] if len(parts) > 5 else "-"
    if hand and hand != "-":
        for ch in hand:
            letter, owner = CHAR[ch]
            (sente_hand if owner == "sente" else gote_hand).append(letter)
    return pieces, sente_hand, gote_hand


def render_from_posstring(code, outpath, highlight=None):
    pieces, sh, gh = parse_posstring(code)
    render_position(pieces, outpath, sente_hand=sh, gote_hand=gh, highlight=highlight)


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
    # found by tools/find_positions.c scanning the tablebase
    render_from_posstring("S/cgl/--e/--L/c-G/E", os.path.join(OUT, "position-173ply.svg"))
    render_from_posstring("S/---/lc-/Eg-/GEL/C", os.path.join(OUT, "position-chickdrop.svg"),
                          highlight=("C", 1))
