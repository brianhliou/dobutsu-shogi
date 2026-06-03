# solver/ — from-scratch Dōbutsu Shōgi tablebase (Rust)

Our own rules engine and (coming) endgame-tablebase solver, built to be validated
position-by-position against clausecker/dobutsu (the oracle), then compiled to WASM
to power the explorer as a static page — and to run the **drops ablation** (re-solve
with drops disabled) that the article's §4 thesis rests on.

## Status

- **Rules engine — done.** Move generation (steps + drops), make-move (captures to
  hand, chick promotion on advance), Try/lion-capture terminal detection, and
  clausecker-format position strings.
- **Bulk oracle diff — done.** `cargo run --release --bin oracle_diff` BFS-checks
  **200,000 positions against clausecker's probe: 0 mismatches.** This surfaced the Try
  rule: a lion reaching the enemy back rank wins only on an *unattacked* square; onto an
  attacked square it is a normal (losing) move. Positions with the lions adjacent or a
  lion ascended are resolution positions clausecker doesn't store standalone — valid as
  children, skipped as probe inputs.
- **Full solve — done.** `cargo run --release --bin solve` enumerates **213,993,386**
  canonical reachable positions (turn + mirror folded) and fills distance-to-mate by
  retrograde analysis (~75 min, ~7 GB). **The initial position evaluates to −78 (Gote wins
  in 78)**, max DTM 173, win/loss/draw 174,089,910 / 37,228,827 / 2,674,649; a 4,911-position
  spot-check against clausecker has **0 mismatches**. Our own tablebase, independently
  computed and validated. (Slow because each round regenerates moves+keys; a cached
  successor adjacency would cut it to ~10 min.)

## Roadmap

1. ~~Bulk oracle diff~~ — **done** (200k positions, 0 mismatches).
2. ~~Enumerate~~ — **done** (213,993,386 canonical positions).
3. ~~Retrograde + validate~~ — **done** (initial = −78, 4,911-position spot-check vs
   clausecker = 0 mismatches).
4. Serialize the tablebase to a file; serve it (Railway probe) and point the explorer at it.
5. Speed: cache successor adjacency (one round = array reads) to cut the solve to ~10 min.
6. **Drops ablation:** a `--no-drops` flag in move generation, re-solve, compare draw
   rate / max-DTM / depth profile against the standard game (the §4 stat).
7. (Optional) compile to WASM for a static/offline explorer.

## Run

```sh
cd solver
cargo test
echo 'S/gle/-c-/-C-/ELG/-' | cargo run --quiet   # legal moves + resulting positions
```

Position-string format (clausecker's): `side / rank1 / rank2 / rank3 / rank4 / hand`,
uppercase = first player, lowercase = second; `R`/`r` = promoted chick (hen).
