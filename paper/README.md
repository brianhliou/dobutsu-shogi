# paper/ — primary source

The canonical reference for this project. The PDF and a working translation are
kept locally but not redistributed here — they're third-party copyrighted
material. This file records the citation and where to read the original.

## Citation

田中哲朗 (Tetsuro Tanaka), 「どうぶつしょうぎ」の完全解析
("An Analysis of a Board Game 'Doubutsu Shogi'", in Japanese),
IPSJ SIG Notes (情報処理学会研究報告), **Vol. 2009-GI-22, No. 3, pp. 1–8 (2009)**.
NII permalink: <http://id.nii.ac.jp/1001/00062415/>

- Author's Dōbutsu Shōgi page: <https://www.tanaka.ecc.u-tokyo.ac.jp/ktanaka/dobutsushogi/>
- Author's publications: <https://www.tanaka.ecc.u-tokyo.ac.jp/ktanaka/papers.html>

## Our independent work

The analysis stands on its own, without reproducing the source:

- `research/reproduction.md` — we regenerated the full tablebase from scratch and
  confirmed Tanaka's headline results (initial position `#-78`, max distance-to-mate
  173 plies), validated against the clausecker/dobutsu engine.
- `solver/` — the from-scratch Rust rules engine, solver, and probe.
- `article/` — the English write-up.
