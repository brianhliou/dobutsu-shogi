//! Count the positions reachable from the start (our reachability definition:
//! expand every move except terminal wins — lion capture / safe Try — which end
//! the game). Uses packed u64 keys to keep memory near the position count.
//!
//! Flags:
//!   --no-drops  no-drops variant (captures leave the board, as in chess)
//!   --raw       count raw reachable positions, no symmetry folding (the basis
//!               of Tanaka's 246,803,167); default folds turn + mirror to the
//!               canonical key (the solver's 213,993,386).
//!
//!   cargo run --release --bin enumerate -- [--no-drops] [--raw]

use std::collections::{HashSet, VecDeque};

use dobutsu::{canonical_key, pack, parse, unpack};

const INIT: &str = "S/gle/-c-/-C-/ELG/-";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let no_drops = args.iter().any(|a| a == "--no-drops");
    let raw = args.iter().any(|a| a == "--raw");

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
    let mut terminal_wins = 0u64;

    while let Some(key) = q.pop_front() {
        let p = unpack(key);
        let ms = if no_drops { p.moves_nd() } else { p.moves() };
        // a position with any terminal-win move (lion capture / safe Try) is itself a
        // win — the game ends there, so it is not expanded (this is also how Tanaka
        // defines reachability; expanding won positions explodes toward all-legal).
        if ms.iter().any(|m| p.is_terminal_win_move(m)) {
            terminal_wins += 1;
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
    let basis = if raw { "raw reachable" } else { "canonical" };
    println!("{variant} / {basis}: {}", seen.len());
    println!("processed:              {processed}");
    println!("terminal-win positions: {terminal_wins}");
}
