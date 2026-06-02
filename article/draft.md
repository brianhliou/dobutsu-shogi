---
layout: post
title: "Solving Dōbutsu Shōgi: How a 3×4 Children's Game Runs So Deep"
date: 2026-06-01
description: "Dōbutsu shōgi is a children's shogi variant that's been strongly solved: the second player wins in 78 plies. How retrograde analysis solves it, and why one rule makes a tiny board so deep."
---

<!-- DRAFT: full piece, beats 1-10. -->

Dōbutsu shōgi ("animal chess") is a shogi variant on a 3×4 board with eight pieces, designed in 2008 by professional player Madoka Kitao to teach shogi to children. Despite the toy-like look, it is a completely solved game.

In 2009, Tetsuro Tanaka computed the exact value of every reachable position. **With perfect play the second player wins, in 78 plies; the first player to move loses.**

This post covers how the solve was done, what it found, and why such a small board produces a deep game. The depth traces to one rule from shogi that most small abstract games omit: captured pieces return to play. (The result here was reproduced from a regenerated tablebase; details below.)

**Source:** Tetsuro Tanaka, "An Analysis of a Board Game 'Doubutsu Shogi'" (IPSJ SIG Notes, Vol. 2009-GI-22, 2009). [NII permalink](http://id.nii.ac.jp/1001/00062415/)

<!-- WIDGET: opening explorer (W1) embeds here once built -->

---

## 1. The game

Each player has four pieces: a **Lion**, a **Giraffe**, an **Elephant**, and a **Chick**, set up as mirror images of each other:

<!-- assets generated in repo at assets/diagrams/; copy to blog /assets/posts/dobutsu-shogi/ on publish -->
<figure style="margin:1.5rem 0;text-align:center">
<img src="/assets/posts/dobutsu-shogi/initial-position.svg" alt="Dōbutsu shōgi starting position on a 3 by 4 board" style="max-width:280px">
<figcaption style="font-size:0.85em;color:#666;margin-top:4px">Starting position (L Lion, G Giraffe, E Elephant, C Chick). Upper pieces point down (second player); lower pieces point up and move first.</figcaption>
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

A captured piece joins the captor's hand and can later be **dropped** onto any empty square as their own. Captured pieces switch sides rather than leaving the board. This drop rule is what keeps the game deep (more below).

---

## 2. What "solved" means

"Solved" comes in three strengths:

| Level | What's known |
|---|---|
| **Ultra-weak** | The result with perfect play (who wins), but not how. |
| **Weak** | A strategy to reach that result from the starting position. |
| **Strong** | The best move from *every* legal position, not just the start. |

Dōbutsu shōgi is **strongly solved**: a lookup table (a *tablebase*) gives the exact value and distance-to-finish for any reachable position, so a program holding that table plays perfectly from anywhere, not just the opening. Chess is solved only for endgames with few pieces left; its opening is far out of reach. Dōbutsu shōgi is small enough for the whole game to fit in one table.

---

## 3. Retrograde analysis

Forward search from the start doesn't work: there's no move limit, lines can repeat, and the tree is too deep to reach the end. The solve runs backward instead, a method called **retrograde analysis**.

It begins from finished positions (a captured Lion, or a successful Try), whose values are known by definition, then steps back one move at a time:

- A position is a win if the player to move can step into a win.
- A position is a loss only if every legal move gives the opponent a win.

This propagates until the win and loss sets stop growing. At that point every position has a value.

Scale is what makes it tractable. Each position fits in **60 bits** (48 for the board, 12 for the pieces in hand), so the full state space stays in memory. **246,803,167** positions are reachable from the start; the backward pass over all of them took about **5.5 hours on a 2009 machine**.

---

## 4. What the solution says

The starting position is a win for the second player, and the reason is **zugzwang**: a position where the side to move would be content to pass, but has to move, and every move makes things worse. At the opening, the first player is already in that bind. Moving is compulsory, no first move improves the position, and best play by the second player converts the disadvantage over the next 78 plies.

Zugzwang recurs throughout the game. Tanaka's pass over the full state space found at least 21,839 zugzwang positions, so the opening is one instance of a pattern that runs everywhere.

---

## 5. Reproducing it

The result is checkable. An open-source engine and tablebase generator exists ([clausecker/dobutsu](https://github.com/clausecker/dobutsu)). I rebuilt the full tablebase from source: under a minute, a 168 MB table, and the engine's validator confirmed it was internally consistent.

Querying the starting position returns `#-78`, meaning the side to move (the first player) is lost in 78 plies. Listing the first player's four legal moves shows three losing in 78 and one losing sooner:

```
Gc4-c3 : #-78
Lb4-c3 : #-78
Lb4-a3 : #-78
Cb3xb2 : #-76
```

`Cb3xb2`, capturing the opponent's chick, is the fastest way to lose. The intuitive "take the free piece" move is the worst of the four.

---

## 6. What else the table shows

A full solution turns up more than the opening result:

- **The longest forced win is 173 plies.** The opening's 78 is far from the worst case; somewhere in the tree sits a won position that takes 173 plies to finish.
- **68 positions have a chick-drop as their only winning move.** Dropping a chick onto the opponent's back row leaves it where it can't advance, so the move barely changes the position, yet in these 68 cases it's the single move that holds the win.
- **About 30% of won positions contain a forced mate**, and the mates stay short: the longest is 23 plies.

---

## 7. Why a tiny board runs deep

Most abstract games simplify as they go. Captures remove material, the board empties, and the endgame is thinner than the middlegame. Shrinking such a game to a tiny board tends to make it shallow: small minichess variants often turn into quick draws once the pieces trade off.

Drops break that pattern. A captured piece returns to play in the captor's hand, switching owner rather than leaving the game. Material never drains away, threats keep regenerating, and the position stays sharp down to the last move. That is how a 3×4 board with eight pieces holds nearly 247 million reachable positions and a 173-ply forced win.

Tanaka reaches the same conclusion: the complexity on so small a board reflects the strength of the drop rule, together with well-chosen piece moves and starting setup. Dōbutsu shōgi keeps shogi's defining mechanic and shrinks everything around it, which is what separates it from a flat minichess.

---

## 8. Still worth playing

A solved game can still be a good game. The perfect lines run dozens of plies deep across hundreds of millions of positions, past what a person can hold in memory, so human games leave the theoretical path within a few moves and the second player's edge usually slips away. Dōbutsu shōgi remains the best-selling shogi product in Japan and a common first board for children. "Solved" describes what a computer knows, not how the game plays across a table.

---

The point generalizes: depth in a small abstract game comes less from board size than from whether material recirculates. A drop rule, or any mechanism that returns captured pieces to play, can keep a tiny game sharp where a pure-subtraction game would collapse into a draw.

**Resources:**
- [Tanaka, "An Analysis of a Board Game 'Doubutsu Shogi'"](http://id.nii.ac.jp/1001/00062415/): the original solve (Japanese).
- [clausecker/dobutsu](https://github.com/clausecker/dobutsu): open-source engine and tablebase generator.
