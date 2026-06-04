---
layout: post
title: "Solving Dōbutsu Shōgi: How a 3×4 Children's Game Runs So Deep"
date: 2026-06-01
description: "Dōbutsu shōgi, a children's shogi variant, is strongly solved: the second player wins in 78 plies. The results of the solve, and why one rule makes a tiny board so deep."
---

<!-- DRAFT: 4 sections. Strip HTML comments on publish. -->
<!-- PUBLISHED 2026-06-04 at https://brianhliou.com/posts/dobutsu-shogi/ (blog repo: brianhliou.github.io,
     _posts/2026-06-04-dobutsu-shogi.md). This file is the canonical authoring source; the published copy
     adds the live explorer embed (https://dobutsu.brianhliou.com/) and uses /assets/projects/dobutsu-shogi/. -->
<!-- Live explorer: https://dobutsu.brianhliou.com/  ·  source assets: assets/diagrams/ -->

Dōbutsu shōgi ("animal chess") is a children's shogi game on a 3×4 board with eight pieces. It has been completely solved: with perfect play, the player who moves first loses.

A professional shogi player, Madoka Kitao, designed it in 2008 to teach the game to children. In 2009, Tetsuro Tanaka computed the exact value of every reachable position. **The second player wins, in 78 plies.**

This post covers what the solution turned up and why such a small board produces a deep game. The depth traces to one rule from shogi that most small abstract games omit: captured pieces return to play.

**Source:** Tetsuro Tanaka, "An Analysis of a Board Game 'Doubutsu Shogi'" (IPSJ SIG Notes, Vol. 2009-GI-22, 2009). [NII permalink](http://id.nii.ac.jp/1001/00062415/)

<!-- WIDGET: opening explorer (W1) embeds here once built -->

---

## 1. The game

Each player has four pieces: a **Lion**, a **Giraffe**, an **Elephant**, and a **Chick**, set up as mirror images of each other.

<!-- assets generated in repo at assets/diagrams/; copy to blog /assets/posts/dobutsu-shogi/ on publish -->
<figure style="margin:1.5rem 0;text-align:center">
<img src="/assets/posts/dobutsu-shogi/initial-position.svg" alt="Dōbutsu shōgi starting position on a 3 by 4 board" style="max-width:330px">
<figcaption style="font-size:0.85em;color:#666;margin-top:4px">Starting position. The first player (bottom, ivory tiles) moves first; the second player (top, dark tiles) mirrors them. The dots on each piece mark the squares it can step to.</figcaption>
</figure>

### Moves

Each piece moves one square per turn:

- **Lion**: any of the 8 directions (a chess king).
- **Giraffe**: one square horizontally or vertically.
- **Elephant**: one square diagonally.
- **Chick**: one square straight forward. Advancing onto the far row promotes it to a **Hen** (one step in any direction except the two back diagonals). A chick *dropped* onto the far row does not promote, so it is stuck there.

<figure style="margin:1.5rem 0;text-align:center">
<img src="/assets/posts/dobutsu-shogi/piece-moves.svg" alt="How each piece moves: dots mark the squares it can step to" style="max-width:640px">
<figcaption style="font-size:0.85em;color:#666;margin-top:4px">Each piece moves one step to a dotted square; a Chick that advances to the far row promotes to a Hen.</figcaption>
</figure>

### Winning

Two ways to win: capture the opponent's Lion, or get your Lion to the enemy's back rank, onto a square the opponent can't immediately capture (a **Try**).

### Drops

A captured piece joins the captor's hand and can later be **dropped** onto any empty square as their own. Captured pieces switch sides rather than leaving the board. This is shogi's signature rule, the one chess lacks.

---

## 2. The result

Dōbutsu shōgi is **strongly solved**: a lookup table (a *tablebase*) holds the exact value of every reachable position, so a program with it plays perfectly from any position, not just the opening. Tanaka built the table by working backward from finished positions, covering every position the game can reach.

The starting position is a **second-player win in 78 plies**. The first player loses to **zugzwang**: every move worsens the position, and passing isn't allowed, so moving first is itself the disadvantage.

Rebuilding the table from an open-source solver ([clausecker/dobutsu](https://github.com/clausecker/dobutsu)) reproduces this. The starting position evaluates to `#-78` (the side to move is lost in 78 plies), and its four legal first moves all lose:

```
Gc4-c3 : #-78
Lb4-c3 : #-78
Lb4-a3 : #-78
Cb3xb2 : #-76
```

The full solution, in numbers:

| Quantity | Value |
|---|---|
| Reachable positions | 246,803,167 |
| Non-terminal positions | 99,485,568 |
| Wins / draws / losses (side to move) | 56,474,473 / 2,682,700 / 40,328,395 |
| Average legal moves per position | 9.4 |

---

## 3. Notable findings

Beyond the headline result, a few things stand out:

- **Wins finish fast, with a long tail.** Most won positions end within 15 plies; the deepest forced win runs 173 plies (far past the opening's 78), reached by only 14 positions.
- **In 68 positions, the only winning move is to drop a chick where it can't move.** A chick on the opponent's back row has no square ahead of it, so the drop barely changes anything, close to passing a turn. Yet in these 68 positions every other move loses, and that near-pass is the one that wins.
- **Often only one move wins.** In about a third of won positions a single move keeps the win; every other move loses it.
- **Zugzwang is common.** The opening isn't a special case: at least 21,839 positions are traps where moving loses but passing, if it were allowed, would hold.
- **About 30% of won positions contain a forced mate**, and the mates stay short: the longest is 23 plies.

<figure style="margin:1.5rem 0;text-align:center">
<img src="/assets/posts/dobutsu-shogi/depth-profile.svg" alt="Log-scale bar chart of won positions by distance to win, decaying from about 10 million at 3 plies to 14 at 173 plies" style="max-width:700px">
<figcaption style="font-size:0.85em;color:#666;margin-top:4px">Won positions by distance to win (log scale). Most resolve in a handful of moves; a thin tail reaches all the way to the 173-ply maximum.</figcaption>
</figure>

<figure style="margin:1.5rem 0;text-align:center">
<img src="/assets/posts/dobutsu-shogi/position-chickdrop.svg" alt="A position whose only winning move is dropping a chick on c1" style="max-width:480px">
<figcaption style="font-size:0.85em;color:#666;margin-top:4px">The only winning move is to drop the in-hand chick on c1 (circled), where it can never advance. Every other move loses.</figcaption>
</figure>

<figure style="margin:1.5rem 0;text-align:center">
<img src="/assets/posts/dobutsu-shogi/position-173ply.svg" alt="A position that is a forced win in 173 plies" style="max-width:430px">
<figcaption style="font-size:0.85em;color:#666;margin-top:4px">The deepest forced win in the game: the side to move (warm pieces) wins in 173 plies, though down material.</figcaption>
</figure>

---

## 4. Why a tiny board runs deep

In most games material only drains away. Captures thin the board, the endgame is simpler than the opening, and a smaller board means a shorter game; minichess variants tend to fizzle into draws.

Drops invert that. A captured piece comes back in the captor's hand, so the eight pieces keep circulating instead of disappearing, and the board never simplifies. It shows in the numbers: in roughly one in six won positions, the side to move wins despite having fewer pieces, so a material lead guarantees nothing here.

To check that drops are the cause, I solved the same game with the rule removed, so captured pieces leave the board as in chess. It collapses. The reachable positions fall from hundreds of millions to under a million, the deepest forced win drops from 173 plies to 37, and the opening is no longer a win at all: with no drops, best play is a draw. The depth was the drop rule the whole time.

---

The point generalizes: depth in a small abstract game comes less from board size than from whether material recirculates. A drop rule, or any mechanism that returns captured pieces to play, can keep a tiny game sharp where a pure-subtraction game would collapse into a draw.

**Resources:**
- [Tanaka, "An Analysis of a Board Game 'Doubutsu Shogi'"](http://id.nii.ac.jp/1001/00062415/): the original solve (Japanese).
- [clausecker/dobutsu](https://github.com/clausecker/dobutsu): open-source engine and tablebase generator.
- Animal glyphs: [Twemoji](https://github.com/jdecked/twemoji), © Twitter, licensed [CC-BY 4.0](https://creativecommons.org/licenses/by/4.0/).
