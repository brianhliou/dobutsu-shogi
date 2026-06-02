# Article outline — "How a Children's Game Got Solved" (working title)

> Planning doc, not prose. When we draft the actual article, invoke the `draft-voice` skill.
> Goal: the rigorous, correct, primary-source-grounded English explainer of Tanaka's solve —
> the piece that currently doesn't exist in English. Distribution: brianhliou.com → Reddit →
> YouTube.

## The angle (what makes this not-a-Wikipedia-rewrite)

1. **We read the Japanese paper.** Almost no English source does. That alone differentiates.
2. **We correct the record.** The most-cited English fact (the position count) is wrong.
   Reachable = 246,803,167, not 1.57 billion. We show *why* the 1.57B is an unreachable upper
   bound. This is the credibility hook and the social-shareable beat.
3. **We explain the depth, not just the result.** Why is a 3×4 toy deep enough to be worth
   solving? Tanaka's own answer: **the drop rule**. That's a satisfying, non-obvious payoff.

## Audience

Primary: chess / game-theory / puzzle-curious general readers (r/chess, r/shogi, HN-adjacent).
Assume zero shogi knowledge; assume some "I've heard a game can be 'solved'" curiosity.

## Beat sheet (draft)

1. **Hook** — the cutest game in Japan (kids' game, animal pieces, designed to bring girls
   into shogi) is also *completely solved*. Set the tension: cute ↔ solved.
2. **The rules in 60 seconds** — 3×4, 8 pieces, Lion/Giraffe/Elephant/Chick, capture-the-Lion
   or Try. Keep it visual; this is where the kid-game charm lands.
3. **What "solved" means** — Win/Loss/Draw is definite for a 2-player zero-sum perfect-info
   game; ultra-weak vs weak vs **strong**. State the precise claim (reachable positions).
4. **The result** — second player wins, in 78 plies. The zugzwang intuition: Sente loses
   *because* Sente must move. Contrast: deepest forced win anywhere is 173 plies.
5. **How Tanaka did it** — retrograde analysis, backward from terminal positions; 60-bit
   encoding; 246,803,167 reachable positions; ~5.5 hours on one 2009 machine. Make
   retrograde analysis intuitive (solve the endgame first, work backward).
6. **The correction** — the 1.57-billion number, where it comes from (Table 1 upper bound),
   why it's not "reachable," and the right number. (Decision: lead with this or place mid-
   piece? — open question.)
7. **The beautiful bits** — the 68 positions where dropping a chick that can't even move is
   the *only* winning move; ~30% of wins hide a mate but the longest is only 23 plies.
8. **Why a toy is deep: drops** — captured pieces come back, the board never empties, the
   game stays sharp. Tie to the broader "small + drops = surprising depth" idea.
9. **Does solved mean dead?** — no: perfect play is beyond human memory; still the best-
   selling shogi product in Japan. Solved ≠ unfun.
10. **Close** — pointer to the source, the tablebase engines, and (eventually) our own
    reproduction / interactive explorer.

## Assets needed

- Clean board diagrams (initial position; a zugzwang; the 68-class chick-drop position; the
  173-ply position). Candidate: render from the paper's figures or generate our own.
- A retrograde-analysis explainer graphic.
- (Stretch) an interactive "play the perfect engine" widget — bridges to the chess-app stack.

## Distribution plan (sketch — flesh out later)

- **brianhliou.com** — canonical long-form home.
- **Reddit** — r/chess, r/shogi, r/boardgames; lead with the "Wikipedia's number is wrong"
  correction as the title hook.
- **YouTube** — a short visual version (the rules + the solve + the chick-drop curiosity).
- Future: HN, X, etc.

## Not in this article (scope discipline)

The "design my own animal game" thread stays out — it's a separate project. This piece is
about the *solve*. Linking to it later is fine; folding it in now dilutes both.
