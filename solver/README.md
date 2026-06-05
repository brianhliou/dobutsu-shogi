# solver/ — from-scratch Dōbutsu Shōgi tablebase (Rust)

Our own rules engine and endgame-tablebase solver, built from scratch and validated
position-by-position against clausecker/dobutsu (the oracle). It computes the complete
tablebase that backs the explorer (served via a probe over the compact 333 MB table)
and runs the **drops ablation** — re-solving with drops disabled — that the article's
§4 thesis rests on.

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
  spot-check against clausecker has **0 mismatches**. (213,993,386 is our canonical count;
  Tanaka's published *reachable* figure is 246,803,167 — a different denominator. See
  [`../research/reproduction.md`](../research/reproduction.md).)
- **Compact tablebase — done.** `compact` packs the solve into a 333 MB `dobutsu.ctb`
  (a minimal perfect hash over the canonical keys + 9-bit distance values, ~400 MB
  resident); `ctbprobe` serves it. This is what the deployed explorer runs.
- **Drops ablation — done.** `solve --no-drops` (captured pieces leave the board, as in
  chess) yields **797,658 positions** (~270× smaller), initial value **0 (draw)**, max DTM
  **37 plies** — direct evidence that the drop rule is what makes the game deep (article §4).

## Binaries

| `cargo run --release --bin …` | does |
|---|---|
| `enumerate`            | enumerate / count the canonical reachable positions |
| `solve`               | retrograde solve (add `--no-drops` for the ablation) |
| `compact`             | pack the solve into the compact `dobutsu.ctb` (MPH + 9-bit values) |
| `tbprobe` / `ctbprobe` | stdin/stdout JSON probe over the records / compact table |
| `oracle_diff`         | bulk-validate move/result generation against clausecker |

## Remaining (optional)

- Compile the probe to WASM for a fully static/offline explorer (no server).
- Speed: cache successor adjacency (a round becomes array reads) to cut the solve to ~10 min.

## Run

```sh
cd solver
cargo test
echo 'S/gle/-c-/-C-/ELG/-' | cargo run --quiet   # legal moves + resulting positions
```

Position-string format (clausecker's): `side / rank1 / rank2 / rank3 / rank4 / hand`,
uppercase = first player, lowercase = second; `R`/`r` = promoted chick (hen).
