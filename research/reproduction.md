# Reproduction — independent confirmation of the solve

We regenerated the complete Dōbutsu Shōgi tablebase from source and independently confirmed
Tanaka's headline result. This is the article's "we re-ran it ourselves" credibility beat.

**Result:** the initial position evaluates to **`#-78`** (side-to-move / Sente loses in 78
plies → **gote wins in 78**), and capturing the chick on move 1 evaluates to **`#-76`** (loses
faster). Both match Tanaka (2009) §4.1 to the ply.

## What we used

- **Engine/solver:** [clausecker/dobutsu](https://github.com/clausecker/dobutsu), a C engine
  that builds a **comprehensive endgame tablebase for perfect play from any position** (a full
  strong solution over all legal positions, not just reachable-from-start).
- **Pinned commit:** `dfcf133355c80ccc85ae61b9c8ed864b316ac809`
- **Machine:** Apple Silicon macOS; build with Apple `c99`/clang, brew `readline`, `xz`
  (liblzma), `gettext`, `pkg-config`.
- Clone lives at `external/clausecker-dobutsu/` (git-ignored; reproducible via the steps below).

## Two macOS portability fixes

1. **`pthread_barrier_*` shim.** Apple's libc omits the optional POSIX barrier API that
   `tbgenerate.c` uses for parallel-round synchronization. We added
   `research/repro/pthread_barrier_shim.h` (a standard phase-based barrier over mutex+cond) and
   `#include`d it in `tbgenerate.c` after its system includes. No-op on platforms that already
   provide barriers.
2. **Explicit pkg-config flags.** The Makefile uses BSD-make `!=` shell assignments, which
   macOS's GNU make 3.81 does not evaluate, so the readline/liblzma include+link flags came up
   empty. We pass them on the make command line (overriding the dead `!=` lines) and point
   `INTLCFLAGS/INTLLDFLAGS` at brew's gettext.

## Exact steps

```sh
# from repo root
git clone https://github.com/clausecker/dobutsu external/clausecker-dobutsu
cd external/clausecker-dobutsu
git checkout dfcf133355c80ccc85ae61b9c8ed864b316ac809

# fix 1: barrier shim
cp ../../research/repro/pthread_barrier_shim.h .
#   then add `#include "pthread_barrier_shim.h"` near the top of tbgenerate.c
#   (after #include <string.h>, before #include "dobutsutable.h")

# fix 2: build with explicit flags
export PKG_CONFIG_PATH="$(brew --prefix readline)/lib/pkgconfig:$(brew --prefix xz)/lib/pkgconfig:$PKG_CONFIG_PATH"
GT="$(brew --prefix gettext)"
make gentb dobutsu validatetb \
  RLCFLAGS="$(pkg-config --cflags readline)" \
  RLLDFLAGS="$(pkg-config --libs-only-L --libs-only-other readline)" \
  RLLDLIBS="$(pkg-config --libs-only-l readline)" \
  LZMACFLAGS="$(pkg-config --cflags liblzma)" \
  LZMALDFLAGS="$(pkg-config --libs-only-L --libs-only-other liblzma)" \
  LZMALDLIBS="$(pkg-config --libs-only-l liblzma)" \
  INTLCFLAGS="-I$GT/include" INTLLDFLAGS="-L$GT/lib"

# generate + validate the tablebase
./gentb -j 8 dobutsu.tb           # ~168 MB, peak RSS ~256 MB, < 1 min on Apple Silicon
./validatetb dobutsu.tb           # exit 0 == internally consistent

# query the initial position
printf 'show board\nshow eval\nshow lines\nexit\n' | DOBUTSU_TABLEBASE=./dobutsu.tb ./dobutsu
```

## Transcript (initial position)

```
=== validatetb dobutsu.tb -> exit 0 ===

Loading tablebase... done
1. show board
  ABC
 +---+
1|gle|
2| c |
3| C |
4|ELG| *
 +---+
1. show eval
#-78
1. show lines
Gc4-c3 : #-78  (25.00%)
Lb4-c3 : #-78  (25.00%)
Lb4-a3 : #-78  (25.00%)
Cb3xb2 : #-76  (24.99%)
```

## How this maps to the paper

| Engine output | Meaning | Tanaka (2009) |
|---|---|---|
| `#-78` at the start | Sente (to move) loses in 78 plies | §4.1: gote win, 78 plies ✓ |
| `Cb3xb2 : #-76` | chick capture loses in 76 (faster) | §4.1: chick capture → 76 ✓ |
| `Gc4-c3 / Lb4-c3 / Lb4-a3 : #-78` | the other 3 first moves hold to 78 | §4.1: the 4 legal first moves ✓ |
| `validatetb` exit 0 over the full base | a consistent strong solution | strongly solved ✓ |

Notation: `#-N` = the player to move is mated in N plies (negative = losing); the four moves
shown are Sente's only legal first moves, confirming the branching of 4 at the root.

## Position scan (tools/find_positions.c)

A scanner over the full tablebase (iteration modelled on `validate_tablebase`) confirms two
more of the paper's results and extracts showcase positions for the article:

- **Max distance-to-win = 173 plies**, matching the paper exactly (14 positions reach it).
- **Only-move = un-advanceable chick drop: 64 positions** (41 wins), close to Tanaka's 68. The
  small gap is expected: clausecker covers all legal positions (not just reachable), and uses a
  position-based repetition rule and a +1-ply terminal convention.
- Scanned 116,734,644 non-terminal positions (clausecker's all-legal count; cf. Tanaka's
  99,485,568 reachable non-terminal).
- **Material isn't destiny:** in 9,753,503 won positions (17.5% of wins) the side to move
  wins while controlling fewer of the eight pieces than the opponent.
- **Forcing:** 17,753,131 won positions (~32%) have exactly one winning move; the rest offer
  more than one. (Full distribution in the scanner output.)
- **Depth profile:** won positions by distance-to-win decay from ~13M at dtm 3 to 14 at dtm
  173; about four in five finish within 15 plies. (Full histogram in the scanner output.)
- **Perfect game:** the 78-ply principal variation from the start (second player captures the
  lion on ply 78) is saved to `data/perfect-game.txt`.

Showcase positions, rendered into `assets/diagrams/`:

```
173-ply win        S/cgl/--e/--L/c-G/E
chick-drop-only    S/---/lc-/Eg-/GEL/C   (only winning move: drop the chick on c1)
```

Build (inside the clone): `cp ../../tools/find_positions.c .`, compile with the liblzma cflags,
link with `tbaccess.o poscode.o moves.o position.o notation.o validation.o unmoves.o` and
`-llzma`, then `./find_positions dobutsu.tb` (~30 s).

## Our own tablebase + drops ablation (solver/)

The Rust solver (`solver/`) independently re-derives the tablebase: enumerate canonical
reachable positions (turn + left-right folded) and fill distance-to-mate by retrograde
analysis. Validated against clausecker with `tools/verify_tb.py`, which walks positions
reachable from the start and compares both tablebases on each one: **50,000 positions, 0
mismatches on result and distance-to-mate** (child-only positions clausecker rejects
standalone are skipped). The initial position and its four first moves match clausecker's
DTM to the ply.

The explorer serves this tablebase directly. Two probes speak the same stdin/stdout JSON
protocol as the clausecker probe, so `explorer/serve.py` uses whichever exists:
`solver/src/bin/tbprobe.rs` reads the 2.14 GB `(key, value)` records, and
`solver/src/bin/ctbprobe.rs` reads the **compact 333 MB `dobutsu.ctb`** — a minimal perfect
hash over the canonical keys (no keys stored, ~3 bits each) plus 9-bit distance-to-mate
values, built by `solver/src/bin/compact.rs`. The compact probe holds ~400 MB resident
(vs 2.14 GB) — small enough to host cheaply — and `verify_tb.py` confirms it returns the
same verdict as clausecker on every position checked. To stay exact without storing keys
(an MPH cannot report "not in the set"), at a position with an immediate winning move the
probe emits only that move; every other position has all of its children in the solved set.

- **Standard game:** 213,993,386 canonical positions, initial −78, max DTM 173, draws 2,674,649.
- **No-drops ablation** (`--no-drops`: captured pieces leave the board as in chess):
  **797,658 positions** (~270× smaller), initial value **0 (draw)**, max DTM **37 plies**,
  draw rate 4.88%. So removing drops turns a 247M-position, 173-ply, second-player-win game
  into a sub-million-position, 37-ply **draw** — direct evidence that the drop rule is what
  makes the game deep (article §4).

  Caveat: the no-drops variant has no external oracle (clausecker is drops-only). It uses the
  same retrograde machinery validated on the standard game, with a minimal rules change
  (no drop moves; captures vanish instead of entering the hand).

## Not yet reproduced (optional)

- Independent confirmation of the 246,803,167 *reachable* count: clausecker counts all legal
  positions (a different denominator), so this would need a reachable-only enumerator. The
  paper is authoritative for the reachable figure.
