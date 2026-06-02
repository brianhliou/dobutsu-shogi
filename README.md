# dobutsu-shogi

A scientific analysis of **Dōbutsu Shōgi** (どうぶつしょうぎ / "Let's Catch the Lion!") and
its complete solution, working from the primary source toward a rigorous, well-cited
**English article** — the explainer that currently does not exist in English.

Dōbutsu Shōgi is a 3×4, 8-piece children's shogi variant designed in 2008 by professional
player Madoka Kitao. It was **strongly solved** by Tetsuro Tanaka (University of Tokyo) in
2009: with perfect play the **second player wins in 78 plies**. The depth of such a tiny
game comes from shogi's **drop rule** (captured pieces return to play) — the same insight
that makes it interesting as a design study.

## Why this repo exists

English coverage of Dōbutsu Shōgi is either (a) rules-and-history SEO, or (b) a terse
Wikipedia paragraph — and that paragraph **states the wrong position count** (it reports
the 1.57-billion *upper bound on all arrangements* as the number of "reachable positions";
the actual reachable count is **246,803,167**). No English-language piece works through
Tanaka's actual analysis. That gap is the wedge: a primary-source-grounded explainer that
gets the numbers right and explains *why* the game is deep.

This repo treats that as a research project, not a blog post: read the source, verify every
claim, log open questions, and (later) run our own experiments.

## Layout

```
paper/
  sources/                     # primary sources (PDFs), committed verbatim
    tanaka-2009-paper.pdf       # the paper — IPSJ SIG-GI, 2009
    tanaka-2009-slides.pdf      # companion slide deck (2009-06-26)
  README.md                    # citation + provenance + reading status
  tanaka-2009-translation.md   # full English translation + our annotations
research/
  findings.md                  # verified-facts ledger (every number, with source)
  open-questions.md            # the scientific question log
article/
  outline.md                   # the English article we're building toward
```

## Status

Foundation laid 2026-06-01. Primary source pulled and fully read; all numbers verified against
the paper; citation, setup, and the Wikipedia-error scope pinned. **Solve independently
reproduced** from the clausecker/dobutsu tablebase — initial position `#-78` (gote win in 78),
chick-capture first move `#-76`, validation clean (see `research/reproduction.md`). Next:
optional experiments (173-ply / chick-drop position queries; drops ablation) and drafting the
article from `article/outline.md`. Open items in `research/open-questions.md`.

## The result, in one paragraph

Dōbutsu Shōgi is a two-player, zero-sum, perfect-information game, so every position has a
definite value. Tanaka enumerated all **246,803,167** positions reachable from the start and
ran **retrograde analysis** (backward induction from terminal positions) to label each
win/loss/draw and its distance-to-result. The initial position is a **second-player (gote)
win requiring 78 plies**; whoever must move first is in zugzwang. Being solved does not make
it unfun to play — perfect lines are well beyond human memory, and the game remains the
best-selling shogi product in Japan.

## Canonical reference

田中哲朗 (Tetsuro Tanaka), 「どうぶつしょうぎ」の完全解析 ("An Analysis of a Board Game
'Doubutsu Shogi'", in Japanese), IPSJ SIG Notes (情報処理学会研究報告), Vol. 2009-GI-22, No. 3,
pp. 1–8 (2009). NII: <http://id.nii.ac.jp/1001/00062415/> ·
Author's page: <https://www.tanaka.ecc.u-tokyo.ac.jp/ktanaka/dobutsushogi/>
