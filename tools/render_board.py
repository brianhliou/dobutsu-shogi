#!/usr/bin/env python3
"""Render Dōbutsu Shōgi positions and a piece-movement legend to SVG.

Pieces are rounded-square tiles carrying an animal glyph plus movement dots (as
on the real game's pieces): a small dot at each edge/corner the piece can step
toward. Owner is shown by tile colour (first player ivory, second player dark
slate); a piece's facing shows in its dots (a chick's dot points at its opponent).
Animal glyphs are Twemoji (CC-BY 4.0), embedded via <defs>/<use>; see
assets/diagrams/emoji/CREDITS.txt.

Positions use the clausecker/dobutsu position string, e.g.
"S/cgl/--e/--L/c-G/E" (side / rank1 / rank2 / rank3 / rank4 / hand).

Usage: python3 tools/render_board.py   ->  assets/diagrams/*.svg
"""

import os
import re
import math

CELL = 96
MARGIN = 34
GAP = 20
HAND_W = CELL
COLS = "ABC"
ROWS = [1, 2, 3, 4]

BG = "#ffffff"
BOARD_FILL = "#f3e7c6"
BOARD_LINE = "#cbb98a"
BOARD_FRAME = "#9c8856"
SENTE_FILL = "#fbf5e6"
SENTE_STROKE = "#6f5d33"
SENTE_DOT = "#7a4a2a"
GOTE_FILL = "#373d47"
GOTE_STROKE = "#20252d"
GOTE_DOT = "#e9eef6"
DOT = "#2b8a8a"
HILITE = "#d98a2b"
LABEL = "#9a8a62"

PIECE = {"L": "lion", "G": "giraffe", "E": "elephant", "C": "chick", "H": "hen"}
CHAR = {"C": ("C", "sente"), "c": ("C", "gote"), "G": ("G", "sente"), "g": ("G", "gote"),
        "E": ("E", "sente"), "e": ("E", "gote"), "L": ("L", "sente"), "l": ("L", "gote"),
        "R": ("H", "sente"), "r": ("H", "gote")}

DIRS = {"N": (0, -1), "S": (0, 1), "E": (1, 0), "W": (-1, 0),
        "NE": (1, -1), "NW": (-1, -1), "SE": (1, 1), "SW": (-1, 1)}
# directions a piece can step, in its own frame (N = toward the opponent)
MOVES = {"L": ["N", "NE", "E", "SE", "S", "SW", "W", "NW"],
         "G": ["N", "E", "S", "W"],
         "E": ["NE", "SE", "SW", "NW"],
         "C": ["N"],
         "H": ["N", "NE", "NW", "E", "W", "S"]}

HERE = os.path.dirname(__file__)
OUT = os.path.join(HERE, "..", "assets", "diagrams")
DATA = os.path.join(HERE, "..", "data")
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


def tile(cx, cy, letter, owner, half=CELL * 0.40, glyph=CELL * 0.50, show_moves=True, shadow=False):
    up = owner == "sente"
    fill, stroke, dotc = (SENTE_FILL, SENTE_STROKE, SENTE_DOT) if up \
        else (GOTE_FILL, GOTE_STROKE, GOTE_DOT)
    filt = ' filter="url(#tsh)"' if shadow else ''
    parts = [f'<rect x="{cx-half:.1f}" y="{cy-half:.1f}" width="{2*half:.1f}" '
             f'height="{2*half:.1f}" rx="{half*0.22:.1f}" fill="{fill}" '
             f'stroke="{stroke}" stroke-width="2"{filt}/>']
    if show_moves:
        sign = 1 if up else -1
        r = half * 0.80   # orthogonal dots at edge midpoints, diagonals at the corners
        for d in MOVES[letter]:
            ux, uy = DIRS[d]
            parts.append(f'<circle cx="{cx+sign*ux*r:.1f}" cy="{cy+sign*uy*r:.1f}" '
                         f'r="{half*0.10:.1f}" fill="{dotc}"/>')
    parts.append(f'<use href="#e_{PIECE[letter]}" x="{cx-glyph/2:.1f}" y="{cy-glyph/2:.1f}" '
                 f'width="{glyph:.1f}" height="{glyph:.1f}"/>')
    return "".join(parts)


def cloud(cx, cy, s):
    return (f'<g fill="#fff" opacity="0.95">'
            f'<ellipse cx="{cx:.1f}" cy="{cy:.1f}" rx="{26*s:.1f}" ry="{15*s:.1f}"/>'
            f'<ellipse cx="{cx-22*s:.1f}" cy="{cy+5*s:.1f}" rx="{17*s:.1f}" ry="{11*s:.1f}"/>'
            f'<ellipse cx="{cx+22*s:.1f}" cy="{cy+5*s:.1f}" rx="{18*s:.1f}" ry="{12*s:.1f}"/>'
            f'<ellipse cx="{cx+2*s:.1f}" cy="{cy-8*s:.1f}" rx="{13*s:.1f}" ry="{9*s:.1f}"/></g>')


# tile-lift (tsh) and board-card (bsh) drop shadows, shared by the boards and
# the move legend so every framed surface reads as the same physical object.
SHADOW_DEFS = (
    '<filter id="tsh" x="-40%" y="-40%" width="180%" height="180%">'
    '<feDropShadow dx="0" dy="1.4" stdDeviation="1.4" flood-color="#000" flood-opacity="0.32"/></filter>'
    '<filter id="bsh" x="-20%" y="-20%" width="140%" height="140%">'
    '<feDropShadow dx="0" dy="3" stdDeviation="5" flood-color="#000" flood-opacity="0.20"/></filter>')


def board_defs():
    """Gradients/filters/clip mirroring the explorer's board, so the static
    diagrams read as frames of the same viewer (explorer/index.html)."""
    return (
        '<defs>'
        '<linearGradient id="sky" x1="0" y1="0" x2="0" y2="1">'
        '<stop offset="0" stop-color="#ddf1fa"/><stop offset="1" stop-color="#cde7f3"/></linearGradient>'
        '<linearGradient id="grass" x1="0" y1="0" x2="0" y2="1">'
        '<stop offset="0" stop-color="#d9ecba"/><stop offset="1" stop-color="#c4dd9d"/></linearGradient>'
        + SHADOW_DEFS +
        f'<clipPath id="bclip"><rect x="{MARGIN}" y="{MARGIN}" width="{3*CELL}" height="{4*CELL}" rx="9"/></clipPath>'
        '</defs>')


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
    s = [svg_open(w, h), f'<rect width="{w}" height="{h}" fill="{BG}"/>', defs(names), board_defs()]
    # field zones (clipped to the rounded board): sky on the top rank, savanna
    # midfield, grass on the home rank, with clouds and a foreground hill —
    # mirrors the explorer so the diagrams read as frames of the live viewer.
    gy = MARGIN + 3 * CELL
    s.append(f'<rect x="{MARGIN}" y="{MARGIN}" width="{3*CELL}" height="{4*CELL}" rx="9" '
             f'fill="{BG}" filter="url(#bsh)"/>')
    s.append('<g clip-path="url(#bclip)">')
    s.append(f'<rect x="{MARGIN}" y="{MARGIN}" width="{3*CELL}" height="{CELL}" fill="url(#sky)"/>')
    s.append(f'<rect x="{MARGIN}" y="{MARGIN+CELL}" width="{3*CELL}" height="{2*CELL}" fill="#f1e6c6"/>')
    s.append(f'<rect x="{MARGIN}" y="{gy}" width="{3*CELL}" height="{CELL}" fill="url(#grass)"/>')
    s.append(cloud(MARGIN + CELL * 0.74, MARGIN + CELL * 0.38, 0.92))
    s.append(cloud(MARGIN + CELL * 2.28, MARGIN + CELL * 0.62, 1.08))
    s.append(f'<g fill="#a6c97c" opacity="0.7">'
             f'<path d="M{MARGIN+CELL*0.35:.1f} {MARGIN+4*CELL} L{MARGIN+CELL*1.05:.1f} {gy+22} '
             f'L{MARGIN+CELL*1.75:.1f} {MARGIN+4*CELL} Z"/>'
             f'<path d="M{MARGIN+CELL*1.55:.1f} {MARGIN+4*CELL} L{MARGIN+CELL*2.3:.1f} {gy+10} '
             f'L{MARGIN+3*CELL+6} {MARGIN+4*CELL} Z"/></g>')
    s.append(f'<path d="M{MARGIN} {MARGIN+4*CELL} L{MARGIN} {gy+46} '
             f'Q{MARGIN+CELL*1.0:.1f} {gy+20} {MARGIN+CELL*1.8:.1f} {gy+40} '
             f'Q{MARGIN+CELL*2.5:.1f} {gy+56} {MARGIN+3*CELL} {gy+34} '
             f'L{MARGIN+3*CELL} {MARGIN+4*CELL} Z" fill="#bcdb92" opacity="0.9"/>')
    s.append('</g>')
    for i in range(1, 3):
        s.append(f'<line x1="{MARGIN+i*CELL}" y1="{MARGIN}" x2="{MARGIN+i*CELL}" '
                 f'y2="{MARGIN+4*CELL}" stroke="#00000018" stroke-width="1"/>')
    for i in range(1, 4):
        s.append(f'<line x1="{MARGIN}" y1="{MARGIN+i*CELL}" x2="{MARGIN+3*CELL}" '
                 f'y2="{MARGIN+i*CELL}" stroke="#00000018" stroke-width="1"/>')
    # frame flush with the field, rounded to match the explorer
    s.append(f'<rect x="{MARGIN}" y="{MARGIN}" width="{3*CELL}" height="{4*CELL}" rx="9" '
             f'fill="none" stroke="{BOARD_FRAME}" stroke-width="3"/>')
    for c, f in enumerate(COLS):
        s.append(f'<text x="{MARGIN+c*CELL+CELL/2:.1f}" y="{MARGIN-8}" font-size="13" '
                 f'text-anchor="middle" fill="{LABEL}" font-weight="700">{f}</text>')
    for r, rk in enumerate(ROWS):
        s.append(f'<text x="{MARGIN-12}" y="{MARGIN+r*CELL+CELL/2+5:.1f}" font-size="13" '
                 f'text-anchor="middle" fill="{LABEL}" font-weight="700">{rk}</text>')
    if highlight:
        hc = MARGIN + COLS.index(highlight[0]) * CELL + CELL / 2
        hr = MARGIN + (highlight[1] - 1) * CELL + CELL / 2
        s.append(f'<circle cx="{hc:.1f}" cy="{hr:.1f}" r="{CELL*0.40:.1f}" fill="{HILITE}" '
                 f'fill-opacity="0.16" stroke="{HILITE}" stroke-width="4"/>')
    for f, rk, letter, owner in pieces:
        cx = MARGIN + COLS.index(f) * CELL + CELL / 2
        cy = MARGIN + ROWS.index(rk) * CELL + CELL / 2
        s.append(tile(cx, cy, letter, owner, shadow=True))
    if has_hand:
        hx = board_right + GAP + HAND_W / 2
        hh, hg, step = CELL * 0.30, CELL * 0.40, CELL * 0.76
        if gote_hand:
            s.append(f'<text x="{hx:.1f}" y="{MARGIN+12}" font-size="12" text-anchor="middle" '
                     f'fill="{LABEL}">in hand</text>')
            for i, x in enumerate(gote_hand):
                s.append(tile(hx, MARGIN + hh + 24 + i * step, x, "gote", half=hh, glyph=hg, show_moves=False, shadow=True))
        if sente_hand:
            base = MARGIN + 4 * CELL
            s.append(f'<text x="{hx:.1f}" y="{base-len(sente_hand)*step-hh-8:.1f}" font-size="12" '
                     f'text-anchor="middle" fill="{LABEL}">in hand</text>')
            for i, x in enumerate(sente_hand):
                s.append(tile(hx, base - hh - i * step, x, "sente", half=hh, glyph=hg, show_moves=False, shadow=True))
    s.append("</svg>")
    write(outpath, "\n".join(s))


def render_moves(outpath):
    panels = [("Lion", "L"), ("Giraffe", "G"), ("Elephant", "E"), ("Chick", "C"), ("Hen", "H")]
    pc, title_h, gap, rx = 40, 28, 26, 4
    grid = pc * 3
    pw, ph = grid, grid + title_h
    w = len(panels) * pw + (len(panels) + 1) * gap
    h = ph + gap * 2
    s = [svg_open(w, h), f'<rect width="{w}" height="{h}" fill="{BG}"/>',
         defs(PIECE[p[1]] for p in panels), '<defs>' + SHADOW_DEFS + '</defs>']
    for i, (name, letter) in enumerate(panels):
        ox, oy = gap + i * (pw + gap), gap + title_h
        s.append(f'<text x="{ox+grid/2:.1f}" y="{gap+16}" font-size="14" '
                 f'text-anchor="middle" fill="#333" font-weight="600">{name}</text>')
        # each panel is a little board: savanna field, #00000018 grid, and the
        # same flush rounded brown frame, so the legend reads as the same surface
        s.append(f'<rect x="{ox}" y="{oy}" width="{grid}" height="{grid}" rx="{rx}" '
                 f'fill="#f1e6c6" filter="url(#bsh)"/>')
        for g in range(1, 3):
            s.append(f'<line x1="{ox+g*pc}" y1="{oy}" x2="{ox+g*pc}" y2="{oy+grid}" '
                     f'stroke="#00000018" stroke-width="1"/>')
            s.append(f'<line x1="{ox}" y1="{oy+g*pc}" x2="{ox+grid}" y2="{oy+g*pc}" '
                     f'stroke="#00000018" stroke-width="1"/>')
        s.append(f'<rect x="{ox}" y="{oy}" width="{grid}" height="{grid}" rx="{rx}" '
                 f'fill="none" stroke="{BOARD_FRAME}" stroke-width="2"/>')
        for d in MOVES[letter]:
            ux, uy = DIRS[d]
            s.append(f'<circle cx="{ox+pc*1.5+ux*pc:.1f}" cy="{oy+pc*1.5+uy*pc:.1f}" '
                     f'r="6.5" fill="{DOT}"/>')
        s.append(tile(ox + pc * 1.5, oy + pc * 1.5, letter, "sente",
                      half=pc * 0.42, glyph=pc * 0.6, show_moves=False, shadow=True))
    s.append("</svg>")
    write(outpath, "\n".join(s))


def render_depth_histogram(csvpath, outpath):
    """Log-scale bar chart: won positions by distance-to-win. Reads dtm,count CSV."""
    data = []
    for line in open(csvpath):
        line = line.strip()
        if line and line[0].isdigit():
            d, c = line.split(",")
            data.append((int(d), int(c)))
    ml, mr, mt, mb = 58, 16, 18, 40
    pw, ph = 660, 240
    w, h = ml + pw + mr, mt + ph + mb
    dmin, dmax = data[0][0], data[-1][0]
    hi = 7.2  # log10 axis top (max count ~1.3e7)
    barw = pw / (len(data) + 2)
    s = [svg_open(w, h), f'<rect width="{w}" height="{h}" fill="{BG}"/>']
    ylabels = {1: "10", 2: "100", 3: "1K", 4: "10K", 5: "100K", 6: "1M", 7: "10M"}
    for k in range(1, 8):
        y = mt + ph * (1 - k / hi)
        s.append(f'<line x1="{ml}" y1="{y:.1f}" x2="{ml+pw}" y2="{y:.1f}" stroke="#eee" stroke-width="1"/>')
        s.append(f'<text x="{ml-8}" y="{y+4:.1f}" font-size="11" text-anchor="end" fill="{LABEL}">{ylabels[k]}</text>')
    for d, c in data:
        x = ml + (d - dmin) / (dmax - dmin) * pw
        bh = ph * math.log10(c) / hi
        s.append(f'<rect x="{x-barw*0.4:.1f}" y="{mt+ph-bh:.1f}" width="{barw*0.8:.1f}" '
                 f'height="{bh:.1f}" fill="{DOT}"/>')
    s.append(f'<line x1="{ml}" y1="{mt+ph}" x2="{ml+pw}" y2="{mt+ph}" stroke="#999" stroke-width="1"/>')
    for d in (3, 25, 50, 75, 100, 125, 150, 173):
        x = ml + (d - dmin) / (dmax - dmin) * pw
        s.append(f'<line x1="{x:.1f}" y1="{mt+ph}" x2="{x:.1f}" y2="{mt+ph+4}" stroke="#999" stroke-width="1"/>')
        s.append(f'<text x="{x:.1f}" y="{mt+ph+18}" font-size="11" text-anchor="middle" fill="{LABEL}">{d}</text>')
    s.append(f'<text x="{ml+pw/2:.1f}" y="{h-3}" font-size="12" text-anchor="middle" fill="#555">distance to win (plies)</text>')
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
    render_depth_histogram(os.path.join(DATA, "depth-profile.csv"), os.path.join(OUT, "depth-profile.svg"))
    # positions found by tools/find_positions.c scanning the tablebase
    render_from_posstring("S/cgl/--e/--L/c-G/E", os.path.join(OUT, "position-173ply.svg"))
    render_from_posstring("S/---/lc-/Eg-/GEL/C", os.path.join(OUT, "position-chickdrop.svg"),
                          highlight=("C", 1))
