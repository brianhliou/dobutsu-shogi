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

## Structure (locked 2026-06-02)

The beat list above is the original sketch; `draft.md` is canonical (now 4 results-first
sections). Two decisions reshape the final layout:

- **Explorer-led.** The interactive tablebase explorer is the marquee artifact and the hook,
  so it leads the post (top embed, Gobblet-style). The §3 showcase positions become
  *explore-this-position* links into it. Implement once the explorer ships.
- **Companion build post.** The from-scratch tablebase (retrograde analysis, encoding, Rust,
  WASM) gets its own writeup, not folded back into this explainer (deliberately trimmed of
  methodology). This article = explainer + explorer; companion = the build.
- **Publish timing.** Polish the text now, but align the public launch with the explorer
  shipping. Don't launch the marquee piece without its marquee artifact.
- When our tablebase exists, update §2's reproduction line: "the tablebase I built reproduces
  this, cross-checked against clausecker (the known solution)."

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
mate). **Decision (2026-06-02): build our own tablebase** (Rust greenfield), validated against
clausecker as an oracle, and make **one explorer that *is* the tablebase** — a full probe: play
from any position, with perfect-move + exact-distance display. (The earlier W1/W2 split
collapses; no toy opening-subset, the explorer exposes the whole solve.) Delivery: WASM
client-side if the table compresses small enough, else a thin probe API.

- **Static diagrams (done).** Bespoke SVG renderer + Twemoji glyphs: initial position, movement
  legend, chick-drop, 173-ply. Stay as fallback / social-card images.
- **The explorer (the marquee artifact).** Leads the post (top embed). The §3 showcase
  positions become *explore-this-position* links into it. No public hosted Dōbutsu tablebase
  exists; this is the novel public contribution.
- Fairy-Stockfish (WASM) is *not* used: it searches, not solves; off-thesis for a solved-game
  explorer.

## Other contributions (beyond the prose)

- **Our own from-scratch tablebase** (Rust), validated position-by-position against clausecker.
  Original engineering; gets its own companion writeup (see Structure).
- **The interactive explorer** — a UI over that tablebase. No public one exists.
- A curated **position dataset** (JSON): the 78-ply line, the 173-ply and chick-drop positions,
  sample zugzwangs — publishable + powers the explorer/diagrams.
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
