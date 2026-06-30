//! Count the positions reachable from the start (our reachability definition:
//! expand every move except terminal wins — lion capture / safe Try — which end
//! the game). Uses packed u64 keys to keep memory near the position count.
//!
//! Flags:
//!   --no-drops  no-drops variant (captures leave the board, as in chess)
//!   --raw       count raw reachable positions, no symmetry folding (default
//!               folds turn + mirror to the canonical key, the solver's
//!               213,993,386).
//!   --tanaka    use Tanaka's terminal convention: a Try resolves one ply later
//!               (+1-ply), so a position is terminal when the side to move can
//!               capture the enemy lion OR the enemy lion already sits on its
//!               home rank. Reproduces Tanaka's 246,803,167 reachable count.
//!
//!   cargo run --release --bin enumerate -- [--no-drops] [--raw] [--tanaka]

use std::collections::{HashSet, VecDeque};

use dobutsu::{canonical_key, pack, parse, unpack};

const INIT: &str = "S/gle/-c-/-C-/ELG/-";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let no_drops = args.iter().any(|a| a == "--no-drops");
    let raw = args.iter().any(|a| a == "--raw");
    let tanaka = args.iter().any(|a| a == "--tanaka");

    let init = parse(INIT).unwrap();
    let mut seen: HashSet<u64> = HashSet::new();
    let mut q: VecDeque<u64> = VecDeque::new();
    let k0 = if raw {
        pack(&init)
    } else {
        canonical_key(&init)
    };
    seen.insert(k0);
    q.push_back(k0);

    let mut processed = 0u64;
    let mut terminal = 0u64;

    while let Some(key) = q.pop_front() {
        let p = unpack(key);
        let ms = if no_drops { p.moves_nd() } else { p.moves() };
        // Terminal positions are not expanded. Default: a move wins on the spot
        // (lion capture or a safe Try). Tanaka's +1-ply convention: only a lion
        // capture is a win, and a position is also terminal (a loss) when the
        // enemy lion already sits on the mover's home rank — one ply after a Try.
        let is_terminal = if tanaka {
            ms.iter().any(|m| p.captures_lion(m)) || p.enemy_lion_on_home_rank()
        } else {
            ms.iter().any(|m| p.is_terminal_win_move(m))
        };
        if is_terminal {
            terminal += 1;
        } else {
            for m in &ms {
                let child = if no_drops { p.make_nd(m) } else { p.make(m) };
                let k = if raw {
                    pack(&child)
                } else {
                    canonical_key(&child)
                };
                if seen.insert(k) {
                    q.push_back(k);
                }
            }
        }
        processed += 1;
        if processed % 10_000_000 == 0 {
            eprintln!(
                "processed {processed:>12}  seen {:>12}  queue {:>11}",
                seen.len(),
                q.len()
            );
        }
        if seen.len() > 500_000_000 {
            eprintln!("ABORT: seen exceeded 500M — reachability is still over-broad");
            std::process::exit(1);
        }
    }

    let variant = if no_drops { "no-drops" } else { "standard" };
    let basis = if raw { "raw" } else { "canonical" };
    let conv = if tanaka { ", tanaka +1-ply" } else { "" };
    println!("{variant} / {basis}{conv}:");
    println!("  total reachable: {}", seen.len());
    println!("  terminal:        {terminal}");
    println!("  non-terminal:    {}", seen.len() as u64 - terminal);
}
