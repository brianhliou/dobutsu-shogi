# CLAUDE.md — dobutsu-shogi

> Research project: analyze Dōbutsu Shōgi and its complete solution from the primary
> source, and produce a rigorous English article. Treat it like science, not blogging.

## What this is (and isn't)

- **Is:** a focused study of Dōbutsu Shōgi — Tanaka's 2009 strong solution, the math behind
  it, and a correct, well-cited English explainer. Distribution targets: brianhliou.com,
  Reddit, YouTube.
- **Isn't (yet):** a game-design project. The "design your own animal game" thread is tracked
  separately and will get its own repo if/when it forms. Keep that out of here unless it
  directly serves the article.

## Working method (the "scientist" contract)

1. **Primary source over secondary.** Tanaka's paper is ground truth. Wikipedia and blogs
   are leads to verify, not facts to cite. When they disagree with the paper, the paper wins
   and the disagreement is itself worth writing about (see the position-count error).
2. **Every number carries its source.** No figure enters `research/findings.md` without a
   page/section pointer into the source. If a number can't be sourced, it's an open question.
3. **Open questions are logged, not guessed.** `research/open-questions.md` is the backlog.
   Resolve them with sources or our own experiments — don't paper over them in prose.
4. **Experiments are reproducible.** If/when we build a solver or playtest harness, it lives
   in this repo with a runnable command and its output committed alongside the claim it backs.

## Conventions

- **Numbers:** write them exactly as the paper does (e.g. 246,803,167). Distinguish *plies*
  (single moves; the paper's 手 / "moves") from *full moves*. The paper counts plies — 78
  plies = 39 each side.
- **Shogi terms:** 先手 = sente = first player; 後手 = gote = second player (the winner here).
  The paper's English abstract renders gote as "white." Be explicit to avoid chess confusion.
- **Pieces:** Lion (玉/king), Giraffe (き, one-step rook / Wazir), Elephant (ぞ, one-step
  bishop / Fers), Chick (ひ, pawn) → promotes to Hen (に, gold-like). Use English names in
  prose, keep the Japanese in parentheses on first use.
- Drafting publishable prose for the article → invoke the `draft-voice` skill first.

## Primary source

田中哲朗, 「どうぶつしょうぎ」の完全解析, IPSJ SIG-GI, 2009.
`paper/sources/tanaka-2009-paper.pdf` · <https://www.tanaka.ecc.u-tokyo.ac.jp/ktanaka/dobutsushogi/>
