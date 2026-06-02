# Article outline

> Planning doc, not prose. When we draft, invoke the `draft-voice` skill.
> Scope: **one article for now** (not a series yet). Goal: the definitive, professional,
> primary-source-grounded English explainer of the Dōbutsu Shōgi solve.

## Working title

**"How Dōbutsu Shōgi Was Solved — and Why a Toy Game Runs So Deep."**
(Contains the empty-slot keywords *Dōbutsu Shōgi* + *solved*; signals the timeless depth angle.)

## Positioning (one paragraph)

We own the empty slot: **the solve, explained and reproduced.** Existing English results are
either Wikipedia's terse paragraph or rules/play pages; none works through *how* the game was
solved or *why* it's deep. We become the depth destination that Wikipedia links out to and that
LLMs cite — authority through doing the work (read the primary source, reproduced the result,
explained it clearly), not through dunking on anyone. Authority + citations first; a
Reddit/HN/YouTube narrative spike drives day-one traffic; SEO long-tail is a bonus.

## SERP read (2026-06, qualitative)

- **Wikipedia ranks #1 on every "solved" query** — unbeatable on the term; shallow on the
  solve. We don't compete for the head term (intent there is play/rules/buy).
- **No dedicated "how it was solved" page exists** — the retrograde-analysis exposition slot
  is empty across all queries checked.
- **Only real content competitor:** `variantslove.netlify.app` — covers *human strategy*, not
  the formal solve/math/reproduction. Different lane, low-authority host.
- **The incorrect position count is propagated everywhere** (Wikipedia, Grokipedia, NamuWiki,
  Kiddle, AI summaries). We cite the correct figure quietly from the paper — not as a hook.
- **YouTube solve-angle is open** — only how-to-play videos exist.

## Spine

> A children's game on a 3×4 board has been completely solved — and the reason it runs so
> deep is a single rule.

## Beats

1. **Hook** — a kids' game that looks trivial but is both deep and fully solved.
2. **Rules in 60 seconds** — 3×4, 8 pieces, capture-the-Lion or Try. Visual; earn the "toy" frame.
3. **What "solved" means** — ultra-weak / weak / **strong**; teach the distinction.
4. **How it's solved** — retrograde analysis made intuitive (solve the endings, walk backward);
   246,803,167 reachable positions; the 60-bit encoding trick.
5. **What the solution says** — second player wins; zugzwang as the intuitive *why* (you lose
   because you must move).
6. **We reproduced it** — rebuilt the full tablebase; initial position `#-78`, chick-capture
   `#-76`. The "an engineer actually ran it" credibility beat.
7. **The surprising results** — the chick-drop-only-win, the 173-ply deepest line, ~30% of
   wins hide a mate (longest 23 plies).
8. **★ The core insight — why a 3×4 toy is deep: drops.** Captured pieces return, the board
   never empties. Tanaka's own conclusion. The transferable game-design lesson. (Centerpiece.)
9. **Solved ≠ dead** — perfect play is beyond human memory; still the best-selling shogi
   product in Japan.
10. **Close** — what the solution reveals about small games; pointer to the source + (eventual)
    interactive explorer.

Beats 6 and 8 are the differentiators only we can write; 8 is the centerpiece.

## Core contributions

- First rigorous **English exposition** of Tanaka (2009).
- **Independent reproduction** of the result.
- **Design synthesis** — small + drops = depth.
- Correct, primary-source figures throughout (part of doing the work, not the pitch).

## Audience & channels

- Audience: chess / game-theory / puzzle-curious general readers; assume zero shogi knowledge.
- Home: brianhliou.com (canonical). Day-one: Reddit (r/chess, r/shogi, r/boardgames), HN.
  Visual cut later: YouTube. SEO long-tail compounds after.

## Diagrams & widgets

Tablebase is the source of truth for every annotation (best move, exact value, distance-to-
mate). Ship in tiers:

- **Static diagrams (with article).** Tiny bespoke SVG renderer for the 3×4 board + our **own**
  piece glyphs, positions generated from the tablebase with exact values baked in. Render:
  initial position, a zugzwang, the chick-drop-only-win, the 173-ply position, a mate example,
  plus a retrograde-analysis explainer graphic.
- **W1 — opening explorer (with article, zero hosting).** Precompute a small JSON of positions
  within ~N plies of the start (exact tablebase values); client-side renderer; click a move →
  perfect move highlighted + exact distance. Provably perfect within the curated tree.
- **W2 — perfect-play probe (stretch; the real linkable asset).** Wrap clausecker's `dobutsu`
  (`setup` → `show eval`/`show lines`) behind a tiny API → "play the perfect engine / paste any
  position, get its exact value + distance." No public hosted Dōbutsu tablebase exists — this
  is the novel artifact.
- Fairy-Stockfish (WASM) only if we later want a beatable free-play opponent without a server.
  It *searches*, it doesn't *solve* — heuristic-strong, not proven; off-thesis for annotations.

## Other contributions (beyond the prose)

- The hosted **perfect-play probe / explorer** (W2) — doesn't exist anywhere.
- A curated **position dataset** (JSON): the 78-ply line, the 173-ply position, the 68 chick-
  drop positions, sample zugzwangs — publishable + powers the diagrams/widgets.
- The **English exposition** of Tanaka (2009).
- Quietly submit the **correct position count to Wikipedia** with the primary-source citation
  (improves the commons; a credibility signal, not the article's hook).

## Tech / IP guardrails

- **Do not** reuse the official animal piece art (copyrighted + trademarked) or the paper's
  figures. Redraw all positions from tablebase data with our own glyphs.
- **clausecker/dobutsu is BSD** — wrapping/hosting it (W2) is license-clean, with attribution.
- pychess/lichess/PlayStrategy board code is **AGPL/GPL** — use only as UX reference; build our
  own renderer (the board is trivial) to avoid copyleft entanglement.
- Reference points: pychess.org supports Dōbutsu via FSF-WASM (good UX study); PlayStrategy
  does **not** have Dōbutsu.

## Out of scope (this article)

- The "design my own animal game" thread (separate project).
- A multi-part "solved small games" series (possible later; not now).
