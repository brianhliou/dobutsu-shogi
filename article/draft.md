---
layout: post
title: "Solving Dōbutsu Shōgi: How a 3×4 Children's Game Runs So Deep"
date: 2026-06-01
description: "Dōbutsu shōgi, a children's shogi variant, is strongly solved: the second player wins in 78 plies. The results of the solve, and why one rule makes a tiny board so deep."
---

<!-- DRAFT: restructured to 5 sections. -->

Dōbutsu shōgi ("animal chess") is a shogi variant on a 3×4 board with eight pieces, designed in 2008 by professional player Madoka Kitao to teach shogi to children.

In 2009, Tetsuro Tanaka computed the exact value of every reachable position. **With perfect play the second player wins, in 78 plies; the first player to move loses.**

This post covers what the solution turned up and why such a small board produces a deep game. The depth traces to one rule from shogi that most small abstract games omit: captured pieces return to play.

**Source:** Tetsuro Tanaka, "An Analysis of a Board Game 'Doubutsu Shogi'" (IPSJ SIG Notes, Vol. 2009-GI-22, 2009). [NII permalink](http://id.nii.ac.jp/1001/00062415/)

<!-- WIDGET: opening explorer (W1) embeds here once built -->

---

## 1. The game

Each player has four pieces: a **Lion**, a **Giraffe**, an **Elephant**, and a **Chick**, set up as mirror images of each other:

<!-- assets generated in repo at assets/diagrams/; copy to blog /assets/posts/dobutsu-shogi/ on publish -->
<figure style="margin:1.5rem 0;text-align:center">
<img src="/assets/posts/dobutsu-shogi/initial-position.svg" alt="Dōbutsu shōgi starting position on a 3 by 4 board" style="max-width:280px">
<figcaption style="font-size:0.85em;color:#666;margin-top:4px">Starting position. The second player (top, cool tiles pointing down) mirrors the first player (bottom, warm tiles pointing up), who moves first.</figcaption>
</figure>

Each piece moves one square per turn:

- **Lion**: any of the 8 directions (a chess king).
- **Giraffe**: one square horizontally or vertically.
- **Elephant**: one square diagonally.
- **Chick**: one square straight forward; on reaching the far row it promotes to a **Hen**, which moves one step in any direction except the two back diagonals.

<figure style="margin:1.5rem 0;text-align:center">
<img src="/assets/posts/dobutsu-shogi/piece-moves.svg" alt="How each piece moves: dots mark the squares it can step to" style="max-width:560px">
<figcaption style="font-size:0.85em;color:#666;margin-top:4px">Each piece moves one step to a dotted square; the Chick promotes to a Hen on the far row.</figcaption>
</figure>

Two ways to win: capture the opponent's Lion, or move a Lion into the far row and survive one turn there (a **Try**).

A captured piece joins the captor's hand and can later be **dropped** onto any empty square as their own. Captured pieces switch sides rather than leaving the board. Chess has no equivalent: this is shogi's signature rule, and it's what keeps the game deep (more below).

---

## 2. The result

Dōbutsu shōgi is **strongly solved**: a lookup table (a *tablebase*) holds the exact value of every reachable position, so a program with it plays perfectly from any position, not just the opening. Chess is solved only for small endgames; checkers was solved in 2007 (a draw). Tanaka built the table by working backward from finished positions, covering every position the game can reach.

The starting position is a **second-player win in 78 plies**. The first player loses to **zugzwang**: every move worsens the position, and passing isn't allowed, so moving first is itself the disadvantage.

Rebuilding the table from an open-source solver ([clausecker/dobutsu](https://github.com/clausecker/dobutsu)) reproduces this. The starting position evaluates to `#-78` (the side to move is lost in 78 plies), and its four legal first moves all lose:

```
Gc4-c3 : #-78
Lb4-c3 : #-78
Lb4-a3 : #-78
Cb3xb2 : #-76
```

`Cb3xb2`, grabbing the opponent's chick, is the fastest way to lose. The natural "take the free piece" move is the worst of the four.

---

## 3. What the table shows

Solving the whole game means every position carries a value, so some aggregate facts fall out:

| Quantity | Value |
|---|---|
| Reachable positions | 246,803,167 |
| Non-terminal positions | 99,485,568 |
| Wins / draws / losses (side to move) | 56,474,473 / 2,682,700 / 40,328,395 |
| Average legal moves per position | 9.4 |

A few results stand out:

- **The longest forced win is 173 plies.** The opening's 78 is far from the worst case; somewhere in the tree sits a won position that takes 173 plies to finish.
- **In 68 positions, the only winning move is to drop a chick where it can't move.** A chick on the opponent's back row has no square ahead of it, so the drop barely changes anything, close to passing a turn. Yet in these 68 positions every other move loses, and that near-pass is the one that wins.
- **Zugzwang is common.** The opening isn't a special case: at least 21,839 positions are traps where moving loses but passing, if it were allowed, would hold.
- **About 30% of won positions contain a forced mate**, and the mates stay short: the longest is 23 plies.

---

## 4. Why a tiny board runs deep

Most abstract games simplify as they go. Captures remove material, the board empties, and the endgame is thinner than the middlegame. Shrinking such a game to a tiny board tends to make it shallow: small minichess variants often turn into quick draws once the pieces trade off.

Drops break that pattern. A captured piece returns to play in the captor's hand, switching owner rather than leaving the game. Material never drains away, threats keep regenerating, and the position stays sharp down to the last move. That is how a 3×4 board with eight pieces holds nearly 247 million reachable positions and a 173-ply forced win.

Tanaka reaches the same conclusion: the complexity on so small a board reflects the strength of the drop rule, together with well-chosen piece moves and starting setup. Dōbutsu shōgi keeps shogi's defining mechanic and shrinks everything around it, which is what separates it from a flat minichess.

---

The point generalizes: depth in a small abstract game comes less from board size than from whether material recirculates. A drop rule, or any mechanism that returns captured pieces to play, can keep a tiny game sharp where a pure-subtraction game would collapse into a draw.

**Resources:**
- [Tanaka, "An Analysis of a Board Game 'Doubutsu Shogi'"](http://id.nii.ac.jp/1001/00062415/): the original solve (Japanese).
- [clausecker/dobutsu](https://github.com/clausecker/dobutsu): open-source engine and tablebase generator.
- Animal glyphs: [Twemoji](https://github.com/jdecked/twemoji), © Twitter, licensed [CC-BY 4.0](https://creativecommons.org/licenses/by/4.0/).
