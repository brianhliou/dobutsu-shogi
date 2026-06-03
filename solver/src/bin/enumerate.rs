//! Count the positions reachable from the start (our reachability definition:
//! expand every move except terminal wins — lion capture / safe Try — which end
//! the game). Uses packed u64 keys to keep memory near the position count.
//!
//!   cargo run --release --bin enumerate

use std::collections::{HashSet, VecDeque};

use dobutsu::{pack, parse, unpack};

const INIT: &str = "S/gle/-c-/-C-/ELG/-";

fn main() {
    let init = parse(INIT).unwrap();
    let mut seen: HashSet<u64> = HashSet::new();
    let mut q: VecDeque<u64> = VecDeque::new();
    let k0 = pack(&init);
    seen.insert(k0);
    q.push_back(k0);

    let mut processed = 0u64;
    let mut terminal_win_edges = 0u64;

    while let Some(key) = q.pop_front() {
        let p = unpack(key);
        for m in p.moves() {
            if p.is_terminal_win_move(&m) {
                terminal_win_edges += 1;
                continue;
            }
            let k = pack(&p.make(&m));
            if seen.insert(k) {
                q.push_back(k);
            }
        }
        processed += 1;
        if processed % 10_000_000 == 0 {
            eprintln!("processed {processed:>12}  seen {:>12}  queue {:>11}", seen.len(), q.len());
        }
    }

    println!("reachable positions: {}", seen.len());
    println!("processed:           {processed}");
    println!("terminal-win edges:  {terminal_win_edges}");
    println!("(Tanaka reports 246,803,167 reachable; ours uses the same stop-at-terminal-win rule)");
}
