# dobutsu-shogi

A scientific analysis of **Dōbutsu Shōgi** (どうぶつしょうぎ / "Let's Catch the Lion!") and
its complete solution, working from the primary source toward a rigorous, well-cited
**English article** — the explainer the topic has lacked.

Dōbutsu Shōgi is a 3×4, 8-piece children's shogi variant designed in 2008 by professional
player Madoka Kitao. It was **strongly solved** by Tetsuro Tanaka (University of Tokyo) in
2009: with perfect play the **second player wins in 78 plies**. The depth of such a tiny
game comes from shogi's **drop rule** (captured pieces return to play) — the same insight
that makes it interesting as a design study.

**Live:** interactive tablebase explorer → **<https://dobutsu.brianhliou.com>** ·
article → **<https://brianhliou.com/posts/dobutsu-shogi/>**

## Why this repo exists

No English-language piece works through Tanaka's actual analysis — coverage is either
rules-and-history or a short encyclopedia summary. This repo fills that gap: a
primary-source-grounded explainer that gets the math right and shows *why* such a small
game runs so deep.

Getting the math right matters, because one figure is easy to conflate. The often-cited
**1,567,925,964** is Tanaka's *upper bound on all piece arrangements ignoring reachability*
(Table 1); the number of positions actually **reachable** from the start is **246,803,167**
(§3.1). The larger number had propagated into several references as the count of "reachable
positions" — we use the correct figure throughout, and contributed the correction, with the
primary-source citation, back to Wikipedia.

This repo treats the work as a research project, not a blog post: read the source, verify
every claim, log open questions, and run our own experiments — a from-scratch tablebase,
reproduced and cross-validated against an independent solver.

## Layout

```
paper/
  README.md          # citation + provenance; source PDF/translation kept local (third-party, not redistributed)
research/
  findings.md        # verified-facts ledger — every number with a page/section source
  open-questions.md  # the scientific question log
  reproduction.md    # how we rebuilt the tablebase and cross-checked it
solver/              # from-scratch Rust rules engine, retrograde solver, tablebase probe
explorer/            # interactive web explorer over the solved tablebase (deployed via Dockerfile)
article/
  draft.md           # the English write-up (canonical authoring source)
data/                # solved-game artifacts (perfect-play line, depth profile)
assets/diagrams/     # diagrams generated from tablebase data
```

## Status

Primary source fully read; every number verified against the paper. The solve was
**independently reproduced** — a from-scratch Rust tablebase (`solver/`), cross-validated
against the clausecker/dobutsu engine: initial position `#-78` (gote win in 78 plies),
chick-capture first move `#-76`, validation clean (see `research/reproduction.md`). An
interactive tablebase **explorer** (`explorer/`) is live (link above); the English article is
in progress. Open items in `research/open-questions.md`.

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
