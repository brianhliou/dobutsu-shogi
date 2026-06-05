# Findings ledger — verified facts

**Primary source:** 田中哲朗 (Tetsuro Tanaka), "An Analysis of a Board Game 'Doubutsu Shogi'"
(Japanese), IPSJ SIG Notes (情報処理学会研究報告), **Vol. 2009-GI-22, No. 3, pp. 1–8 (2009)**.
NII permalink: <http://id.nii.ac.jp/1001/00062415/> . Single entry; no later journal version.

Every number here is sourced to Tanaka (2009) unless noted. Format: claim — value — source.
If a claim isn't sourced, it belongs in `open-questions.md`, not here.

## The result

- **Game class** — two-player, zero-sum, perfect-information; every position has a definite
  value (Win/Loss/Draw). — §abstract, §1
- **Initial position value** — **gote (second player) wins**. — §4.1
- **Plies to win from start** — **78**. — §abstract, §4.1, §5
- **Solution method** — retrograde analysis (backward induction from terminal positions) over
  all positions reachable from the start. — §abstract, §3.2
- **Solve hardware/time** — 16 GB RAM, 2.6 GHz Opteron; enumeration ~19 min, retrograde
  analysis ~5.5 hr. — §3.1, §3.2
- **Independent reproduction (2026-06-01)** — clausecker/dobutsu full tablebase (SHA dfcf133):
  initial position `#-78` (gote win in 78 plies), chick-capture first move `#-76`; `validatetb`
  exit 0; tablebase ~168 MB, <1 min, ~256 MB peak on Apple Silicon. — `reproduction.md`

## Position counts (the disambiguation)

- **Upper bound, all arrangements ignoring reachability** — **1,567,925,964** (Table 1 sum).
  Broken out by pieces-in-hand: 0→638,668,800; 1→638,668,800; 2→242,161,920; 3→44,098,560;
  4→4,134,240; 5→190,080; 6→3,564. — §2, Table 1
  - 0-in-hand board count derivation: 132 × 180 × 112 × 240 = 638,668,800. — §2
- **Reachable from initial position** — **246,803,167**. — §3.1
  - Our independent solve folds turn + left-right symmetry to **213,993,386** canonical positions —
    a different denominator, not a contradiction (the draw subtotal 2,674,649 ≈ Tanaka's non-terminal
    draws 2,682,700 corroborates it). Exact correspondence to 246,803,167 is open (`reproduction.md`).
    **Use 246,803,167 (Tanaka's reachable count) as the public-facing headline figure everywhere.**
- **Non-terminal reachable positions** — **99,485,568** (>half of reachable are terminal). — §3.1
- **Non-terminal value split (side to move)** — win 56,474,473 / draw 2,682,700 / loss
  40,328,395 (sums to 99,485,568 ✓). — §3.2

> ⚠️ The figure **1,567,925,964** has circulated (including, until recently, in English
> Wikipedia) as the number of "reachable positions." Per Table 1 it is the upper bound on
> **all arrangements ignoring reachability**; the reachable count is **246,803,167**. Scope any
> correction to the position count only — the opening-move and 78-ply facts are correct.

## Structure / encoding

- **Position encoding** — 60 bits: 48 (board, 4 bits × 12 squares, 11 states/square) + 12
  (hands, 2 bits × 6) ; side-to-move fixed by symmetry; left-right mirror normalized to the
  smaller 64-bit value. — §3.1
- **Symmetries used** — turn symmetry (fix side to move; 180° rotate if needed) + left-right
  mirror (≈ halves the work). — §2, §3.1

## Other quantitative results

- **Max distance-to-win, any position** — **173 plies** (Fig. 5). — §3.2
- **Average branching (non-terminal)** — **9.435**; by value: win 10.63 / draw 8.82 / loss
  7.81. — §4.2
- **Max branching** — **38** (34 positions, all wins, Fig. 7). — §4.2
- **Min branching** — non-terminal 0-legal-move positions exist but are unreachable → 0 in the
  reachable set; stalemate never occurs in real play. — §4.2
- **Zugzwang positions** — **≥ 21,839** (lower bound; 71 self-symmetric). — §4.3
- **Enemy-zone chick-drop is the unique correct move** — **68 positions**; forbidding the move
  changes 4,301 positions' values but not the initial position's (still 78 plies). — §4.4
- **Tsume (mate) positions** — **17,213,997** total (~30% of wins); longest mate **23 plies**;
  counts by length in Table 3 (translation doc). — §4.5

## Opening lines (§4.1)

Sente's 4 legal first moves and gote's winning replies:

| Sente 1st move | Gote reply | Gote wins in |
|---|---|---|
| B2 Chick (capture) | 同ぞう (recapture, Elephant) | **76** plies (fastest loss for Sente) |
| C3 Giraffe | A2 Giraffe | 78 |
| C3 Lion | A2 Giraffe / B3 Chick | 78 |
| A3 Lion | A2 Giraffe / B3 Chick / C2 Lion | 78 / 78 / 82 |

> Note: capturing the chick **loses fastest** (76 < 78). English Wikipedia states this
> correctly ("...capturing the chick delays loss by only 76 plies"). The inverted "best
> opening move = capture the chick" came from an AI search summary, **not** Wikipedia, so it
> should not be attributed to Wikipedia.

## Reconstructed initial setup (from §4.1 move list)

- Sente (bottom): A4 Elephant, B4 Lion, C4 Giraffe, B3 Chick.
- Gote (top): A1 Giraffe, B1 Lion, C1 Elephant, B2 Chick.
- **Confirmed** by two independent sources: English Wikipedia ("Giraffe to the right of the
  king, Elephant to the left") and the clausecker/dobutsu README board diagram (row 4 `ELG`,
  row 1 `gle`). Both agree with the move-list reconstruction (bottom player's right = column
  C → giraffe C4).

## External classification

- **Solved-ness** — "**strongly solved**." Precise nuance: Tanaka's 2009 paper solved all
  positions *reachable from the start* (every position arising in real play has a known
  optimal move); some legal-but-unreachable positions (e.g. Fig. 4) were excluded. A full
  tablebase engine — clausecker/dobutsu — achieves "perfect play from **any** position," i.e.
  covers all legal positions (true strong solution). State both precisely in the article.
- **Inventor** — Madoka Kitao (women's pro), 2008; piece art by Maiko Fujita. — §1
