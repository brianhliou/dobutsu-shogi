# solver/ — from-scratch Dōbutsu Shōgi tablebase (Rust)

Our own rules engine and (coming) endgame-tablebase solver, built to be validated
position-by-position against clausecker/dobutsu (the oracle), then compiled to WASM
to power the explorer as a static page — and to run the **drops ablation** (re-solve
with drops disabled) that the article's §4 thesis rests on.

## Status

- **Rules engine — done.** Move generation (steps + drops), make-move (captures to
  hand, chick promotion on advance, Try/lion-capture semantics left to the solver),
  and clausecker-format position strings. Validated against the oracle on the opening
  (`cargo test`): the four first moves and their resulting positions match byte-for-byte.

## Roadmap

1. Bulk oracle diff: feed many positions through both `dobutsu-moves` and clausecker's
   probe, assert identical move/child sets.
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
