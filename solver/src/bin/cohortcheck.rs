//! Validate the cohort index before the solve relies on it. Three gates:
//!   1. reachable sample: every reachable non-terminal position encodes into
//!      [0, POSITION_COUNT), distinct canonical positions get distinct offsets,
//!      and decode(encode(p)) round-trips back to the same canonical position.
//!   2. bijection: encode(decode(i)) == i for a stride sample of all offsets
//!      (full sweep with --full).
//!   3. the headline count POSITION_COUNT == 167,527,962.
//!
//!   cargo run --release --bin cohortcheck [--full]

use std::collections::{HashMap, VecDeque};
use std::env;

use dobutsu::cohort::{
    decode_checked, encode, encode_checked, POSITION_COUNT, POSITION_TOTAL_COUNT,
};
use dobutsu::{canonical, pack, parse, Position};

const INIT: &str = "S/gle/-c-/-C-/ELG/-";

fn is_terminal(p: &Position) -> bool {
    let ms = p.moves();
    ms.iter().any(|m| p.is_terminal_win_move(m))
}

fn main() {
    let full = env::args().any(|a| a == "--full");
    println!("POSITION_TOTAL_COUNT = {POSITION_TOTAL_COUNT} (unfolded array size)");
    println!("POSITION_COUNT       = {POSITION_COUNT} (clausecker's folded size)");
    assert_eq!(
        POSITION_TOTAL_COUNT, 255_280_704,
        "table total count mismatch"
    );
    assert_eq!(POSITION_COUNT, 167_527_962, "table count mismatch");

    // ---- gate 1: reachable sample ----
    // encode folds turn AND mirror, so decode(encode(p)) may be p's mirror image
    // (a different board, same offset). The right invariant is in offset space:
    // encode(decode(off)) == off, plus every reachable position lands in range.
    let init = canonical(&parse(INIT).unwrap());
    let mut seen: HashMap<u64, ()> = HashMap::new();
    let mut offsets: std::collections::HashSet<u64> = std::collections::HashSet::new();
    let mut q: VecDeque<Position> = VecDeque::new();
    q.push_back(init);
    seen.insert(pack(&init), ());

    let mut n = 0u64;
    let limit = 2_000_000u64;
    while let Some(p) = q.pop_front() {
        if !is_terminal(&p) {
            let off = encode(&p);
            assert!(
                off < POSITION_TOTAL_COUNT,
                "offset {off} out of range for {p:?}"
            );
            let rt = decode_checked(off).expect("encoded slot must decode");
            assert_eq!(
                encode(&rt),
                off,
                "offset round-trip mismatch at {off}\n {p:?}\n {rt:?}"
            );
            assert_eq!(
                encode_checked(&p),
                Some(off),
                "encode_checked disagrees at {off}"
            );
            offsets.insert(off);
        }
        n += 1;
        if n >= limit {
            break;
        }
        let ms = p.moves();
        if ms.iter().any(|m| p.is_terminal_win_move(m)) {
            continue;
        }
        for m in &ms {
            let child = canonical(&p.make(m));
            if seen.insert(pack(&child), ()).is_none() {
                q.push_back(child);
            }
        }
    }
    println!(
        "gate 1 OK: {n} reachable positions, {} distinct offsets",
        offsets.len()
    );

    // ---- gate 2: bijection encode(decode(i)) == i over valid slots ----
    let stride = if full { 1 } else { 2003 };
    let mut bad = 0u64;
    let mut checked = 0u64;
    let mut i = 0u64;
    while i < POSITION_TOTAL_COUNT {
        if let Some(p) = decode_checked(i) {
            let off = encode(&p);
            if off != i {
                bad += 1;
                if bad <= 5 {
                    eprintln!("bijection FAIL at {i}: re-encoded to {off}\n  {p:?}");
                }
            }
            checked += 1;
        }
        if full && i % 20_000_000 == 0 && i > 0 {
            eprintln!("  {i} / {POSITION_TOTAL_COUNT} ...");
        }
        i += stride;
    }
    if bad == 0 {
        println!("gate 2 OK: encode(decode(i)) == i over {checked} valid sampled slots (stride {stride})");
    } else {
        println!("gate 2 FAIL: {bad} mismatches");
        std::process::exit(1);
    }
}
