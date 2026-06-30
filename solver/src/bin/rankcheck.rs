//! Validate the position rank: confirm `rank(unrank(i)) == i` for every index in
//! `[0, count)`, i.e. rank/unrank are inverse over the whole space (a bijection).
//! Combined with the reachable-sample round-trip test, this proves the index is
//! sound before the solve relies on it.
//!
//!   cargo run --release --bin rankcheck

use dobutsu::rank::Ranker;

fn main() {
    let r = Ranker::new();
    println!("rank count = {}", r.count);

    let mut bad = 0u64;
    let mut i = 0u64;
    while i < r.count {
        if r.rank(&r.unrank(i)) != i {
            bad += 1;
            if bad <= 5 {
                eprintln!("MISMATCH at {i}");
            }
        }
        if i % 100_000_000 == 0 && i > 0 {
            eprintln!("  {i} / {} ...", r.count);
        }
        i += 1;
    }

    if bad == 0 {
        println!("OK: rank/unrank is a bijection over [0, {})", r.count);
    } else {
        println!("FAIL: {bad} mismatches");
        std::process::exit(1);
    }
}
