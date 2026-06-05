# Open questions

The backlog. Each gets resolved by a source or by our own experiment — not by guessing in
prose. Resolved items drop into the log at the bottom (with their resolution).

## Still open

### For our own experiments
- [ ] **Independent confirmation of the 246,803,167 *reachable* count.** clausecker's tablebase
      counts *all legal* positions (a different denominator), so it doesn't directly reproduce
      the reachable figure. Would need a reachable-only enumerator, or treat the paper as
      authoritative for this number. Lower priority.
- [x] **Query the 173-ply deepest win and the chick-drop positions** — done via
      `tools/find_positions.c`. Confirmed max-dtm = 173 (14 positions); found 64 only-move
      chick-drops (cf. Tanaka's 68). Both rendered into `assets/diagrams/`. See
      `research/reproduction.md`.
- [ ] **Drops ablation, quantified.** Tanaka shows forbidding *enemy-zone chick-drops* changes
      4,301 positions. The bigger design question (from the chess note): how much shallower is
      the game with *no drop rule at all*? Our own experiment — needs a modified solver.
- [ ] **Goro Goro / extended variants.** 5×6 "Goro Goro Dōbutsu Shōgi" and the 拡張どうぶつ将棋
      analyses (e.g. the 2013 Kindai University thesis). Solved? By whom? Different result?

## Resolved log

- [x] **Citation** — Tetsuro Tanaka, "An Analysis of a Board Game 'Doubutsu Shogi'" (Japanese),
      IPSJ SIG Notes, **Vol. 2009-GI-22, No. 3, pp. 1–8 (2009)**; NII
      <http://id.nii.ac.jp/1001/00062415/>. (Tanaka's publications page.)
- [x] **Later/journal version?** — None. Single SIG technical report; no journal/proceedings
      version on his publications list.
- [x] **Initial setup orientation** — Sente A4 Elephant / B4 Lion / C4 Giraffe / B3 Chick;
      Gote the 180° mirror. Confirmed by English Wikipedia ("Giraffe right of king, Elephant
      left") **and** the clausecker README board diagram (`ELG` / `gle`).
- [x] **Wikipedia opening-move wording** — Wikipedia is **correct** ("capturing the chick
      delays loss by only 76 plies"). The inverted "best opening move = capture chick" came
      from an AI search summary, not Wikipedia. **Only the position count (1,567,925,964 as
      "reachable") is wrong** — scope the article's correction to that single fact.
- [x] **Plies vs moves** — 手 = ply throughout. The engine reports `#-N` in plies; init `#-78`,
      chick-capture `#-76` match the paper's 78/76. Table 3's odd-only mate lengths confirm.
- [x] **Reproduce the headline** — clausecker/dobutsu full tablebase (SHA dfcf133): initial
      position `#-78` (gote win in 78), chick capture `#-76`, `validatetb` exit 0. See
      `reproduction.md`.
- [x] **"Strongly solved" vs "solved for reachable positions"** — clausecker builds a full
      tablebase ("perfect play from *any* position", validated) = a strong solution over all
      legal positions. Tanaka's 2009 paper solved the reachable subset (every position arising
      in play). Both are accurate; state the distinction precisely.
