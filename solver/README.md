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
- **Dense cohort solve — done.** `solvedense` ports clausecker's computed position index
  (ownership × cohort × lion × placement) and solves over a flat **243 MB** `Vec<i8>` in
  **~3.5 min** instead of the hash map's ~8 GB / ~75 min — smaller *and* faster, since
  gigabyte-scale random access is cache-hostile. `cohortcheck` proves the index is a bijection
  over all 249,442,767 valid slots; the solve matches clausecker's probe with **0 mismatches**.
- **Compact tablebase — done.** `compact` packs the solve into a 333 MB `dobutsu.ctb`
  (a minimal perfect hash over the canonical keys + 9-bit distance values, ~400 MB
  resident); `ctbprobe` serves it. This is what the deployed explorer runs.
- **Drops ablation — done.** `solve --no-drops` (captured pieces leave the board, as in
  chess) yields **797,658 positions** (~270× smaller), initial value **0 (draw)**, max DTM
  **37 plies** — direct evidence that the drop rule is what makes the game deep (article §4).

## Resource footprint

| Run / artifact | Resource use | Time | Output |
|---|---:|---:|---:|
| Standard solve (`solve --save`) | ~7 GB RAM | ~75 min serial, ~17 min parallel | `dobutsu.tb.bin`, 2,139,933,860 bytes (2.0 GiB) |
| Dense cohort solve (`solvedense`) | 243 MB array, 635 MB peak RSS | ~3.5 min | in-memory DTM, all legal positions, 0 mismatches vs probe |
| Compact tablebase (`compact`) | compact build RSS/timing not recorded | not recorded | `dobutsu.ctb`, 332,892,892 bytes (317 MiB) |
| Hosted probe (`ctbprobe`) | ~400 MB resident | cold load in well under the API timeout locally | serves the 333 MB compact table |
| clausecker baseline (`gentb -j 8`) | ~256 MB peak RSS | <1 min on Apple Silicon | `dobutsu.tb`, 167,527,962 bytes (160 MiB) |

The compact file is `92,150,304` bytes of serialized minimal perfect hash plus
`240,742,560` bytes of 9-bit packed distance values and a 28-byte header.

## Binaries

| `cargo run --release --bin …` | does |
|---|---|
| `enumerate`            | enumerate / count the canonical reachable positions |
| `solve`               | retrograde solve over a hash map (add `--no-drops` for the ablation) |
| `solvedense`          | retrograde solve over the dense cohort index (243 MB, ~3.5 min) |
| `cohortcheck`         | prove the cohort index is a bijection over all valid slots |
| `compact`             | pack the solve into the compact `dobutsu.ctb` (MPH + 9-bit values) |
| `tbprobe` / `ctbprobe` | stdin/stdout JSON probe over the records / compact table |
| `oracle_diff`         | bulk-validate move/result generation against clausecker |

## Remaining (optional)

- Compile the probe to WASM for a fully static/offline explorer (no server).
- Port clausecker's Sente≥Gote ownership fold to bring the dense solve from 243 MB to his exact
  167 MB (store 42 of 64 ownership classes; compute the other 22 on demand from children).

## Run

```sh
cd solver
cargo test
echo 'S/gle/-c-/-C-/ELG/-' | cargo run --quiet   # legal moves + resulting positions
```

Position-string format (clausecker's): `side / rank1 / rank2 / rank3 / rank4 / hand`,
uppercase = first player, lowercase = second; `R`/`r` = promoted chick (hen).
