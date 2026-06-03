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

## Roadmap

1. ~~Bulk oracle diff~~ — **done** (200k positions, 0 mismatches).
2. Enumerate reachable positions from the start.
3. Retrograde analysis → distance-to-mate for every position; validate the whole table
   against clausecker.
4. Compile to WASM; swap the explorer's backend from the local probe to in-browser WASM.
5. **Drops ablation:** a `--no-drops` flag in move generation, re-solve, compare draw
   rate / max-DTM / depth profile against the standard game.

## Run

```sh
cd solver
cargo test
echo 'S/gle/-c-/-C-/ELG/-' | cargo run --quiet   # legal moves + resulting positions
```

Position-string format (clausecker's): `side / rank1 / rank2 / rank3 / rank4 / hand`,
uppercase = first player, lowercase = second; `R`/`r` = promoted chick (hen).
