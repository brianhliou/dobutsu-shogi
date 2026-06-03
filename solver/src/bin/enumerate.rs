//! Count the positions reachable from the start (our reachability definition:
//! expand every move except terminal wins — lion capture / safe Try — which end
//! the game). Uses packed u64 keys to keep memory near the position count.
//!
//!   cargo run --release --bin enumerate

use std::collections::{HashSet, VecDeque};

use dobutsu::{canonical_key, parse, unpack};

const INIT: &str = "S/gle/-c-/-C-/ELG/-";

fn main() {
    let init = parse(INIT).unwrap();
    let mut seen: HashSet<u64> = HashSet::new();
    let mut q: VecDeque<u64> = VecDeque::new();
    let k0 = canonical_key(&init);
    seen.insert(k0);
    q.push_back(k0);

    let mut processed = 0u64;
    let mut terminal_wins = 0u64;

    while let Some(key) = q.pop_front() {
        let p = unpack(key);
        let ms = p.moves();
        // a position with any terminal-win move (lion capture / safe Try) is itself a
        // win — the game ends there, so it is not expanded (this is also how Tanaka
        // defines reachability; expanding won positions explodes toward all-legal).
        if ms.iter().any(|m| p.is_terminal_win_move(m)) {
            terminal_wins += 1;
        } else {
            for m in &ms {
                let k = canonical_key(&p.make(m));
                if seen.insert(k) {
                    q.push_back(k);
                }
            }
        }
        processed += 1;
        if processed % 10_000_000 == 0 {
            eprintln!("processed {processed:>12}  seen {:>12}  queue {:>11}", seen.len(), q.len());
        }
        if seen.len() > 500_000_000 {
            eprintln!("ABORT: seen exceeded 500M — reachability is still over-broad");
            std::process::exit(1);
        }
    }

    println!("reachable positions: {}", seen.len());
    println!("processed:           {processed}");
    println!("terminal-win positions: {terminal_wins}");
    println!("(Tanaka reports 246,803,167 reachable)");
}
