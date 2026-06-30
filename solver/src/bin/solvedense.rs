//! Dense-index solve: the same retrograde analysis as `solve`, but storing the
//! distance-to-mate in a flat `Vec<i8>` indexed by the cohort poscode instead of
//! a `HashMap` keyed by the packed position. One byte per slot over the unfolded
//! 255,280,704-slot space (~255 MB resident) replaces the ~8 GB hash map.
//!
//! DTM is stored in *double moves* (clausecker's convention) so it fits a signed
//! byte: a win value `e>0` means win in `2e-1` plies, a loss `e<0` means loss in
//! `-2e` plies, `0` is a draw. The initial position comes out e = -39 (= -78
//! plies, Gote wins). Validated against clausecker's probe.
//!
//!   cargo run --release --bin solvedense

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use dobutsu::cohort::{
    decode_checked, encode, encode_checked, is_valid_offset, POSITION_TOTAL_COUNT,
};
use dobutsu::{canonical, format, parse, Position};
use rayon::prelude::*;

const INIT: &str = "S/gle/-c-/-C-/ELG/-";
const UNKNOWN: i8 = i8::MIN;

/// Evaluate one stored position from prior-round table values. Returns the
/// position's double-move DTM (Some) once decided, or None while still unknown.
/// Values are from the side-to-move's (Sente's, after normalization) viewpoint.
#[inline]
fn evaluate(p: &Position, vals: &[i8]) -> Option<i8> {
    let moves = p.moves();
    // an immediate win (capture the enemy lion, or a surviving ascension)
    if moves.iter().any(|m| p.is_terminal_win_move(m)) {
        return Some(1);
    }

    let mut best_win: Option<i32> = None; // sente wins; track fastest (smallest e)
    let mut worst_loss: Option<i32> = None; // sente loses; track slowest opp win
    let mut any_unknown = false;
    let mut any_draw = false;

    for m in &moves {
        let c = p.make(m);
        // cv = value from the child's mover (the opponent) viewpoint. A child
        // with the lions adjacent (encode_checked == None) is an immediate win
        // for that mover, value +1.
        let cv: i32 = match encode_checked(&c) {
            None => 1,
            Some(off) => {
                let v = vals[off as usize];
                if v == UNKNOWN {
                    any_unknown = true;
                    continue;
                }
                v as i32
            }
        };
        if cv < 0 {
            // opponent loses => sente wins; e = 1 - cv (>= 2)
            let cand = 1 - cv;
            best_win = Some(best_win.map_or(cand, |b| b.min(cand)));
        } else if cv == 0 {
            any_draw = true;
        } else {
            worst_loss = Some(worst_loss.map_or(cv, |w| w.max(cv)));
        }
    }

    let e = if let Some(w) = best_win {
        w
    } else if any_unknown {
        return None;
    } else if any_draw {
        0
    } else if let Some(l) = worst_loss {
        -l
    } else {
        -1 // no legal moves: mate
    };
    Some(e as i8)
}

/// signed plies for comparison with clausecker's probe (get_dtm with sign)
#[inline]
fn signed_plies(e: i8) -> i32 {
    let e = e as i32;
    if e > 0 {
        2 * e - 1
    } else {
        2 * e
    }
}

fn main() {
    let t0 = std::time::Instant::now();
    let n = POSITION_TOTAL_COUNT as usize;
    eprintln!("dense solve: {n} slots (~{} MB)", n / (1 << 20));

    let mut vals = vec![UNKNOWN; n];

    // ---- init: mark immediate wins (+1) in place ----
    // These (the enemy lion capturable now, or a surviving ascension) are the
    // bulk of decided positions; doing them here keeps them out of the fixpoint's
    // per-round result buffer. Disjoint chunks => no aliasing.
    let chunk = 1usize << 20;
    vals.par_chunks_mut(chunk).enumerate().for_each(|(ci, slice)| {
        let base = (ci * chunk) as u64;
        for (i, slot) in slice.iter_mut().enumerate() {
            let off = base + i as u64;
            if let Some(p) = decode_checked(off) {
                if p.moves().iter().any(|m| p.is_terminal_win_move(m)) {
                    *slot = 1;
                }
            }
        }
    });
    let init_wins = vals.par_iter().filter(|&&v| v == 1).count();
    eprintln!("[{:?}] init: {init_wins} immediate wins", t0.elapsed());

    // ---- retrograde fixpoint (Jacobi: decide from prior-round values) ----
    let mut round = 0;
    loop {
        round += 1;
        let decided: Vec<(u32, i8)> = (0..POSITION_TOTAL_COUNT)
            .into_par_iter()
            .filter_map(|off| {
                if vals[off as usize] != UNKNOWN {
                    return None;
                }
                let p = decode_checked(off)?; // skip invalid-ownership slots
                evaluate(&p, &vals).map(|v| (off as u32, v))
            })
            .collect();
        let count = decided.len();
        for (off, v) in &decided {
            vals[*off as usize] = *v;
        }
        if round % 10 == 1 || count == 0 {
            eprintln!("[{:?}] round {round}: decided {count}", t0.elapsed());
        }
        if count == 0 {
            break;
        }
    }
    eprintln!("[{:?}] fixpoint converged after {round} rounds", t0.elapsed());

    // remaining unknown valid slots are draws
    vals.par_iter_mut().for_each(|v| {
        if *v == UNKNOWN {
            *v = 0;
        }
    });

    // ---- report ----
    let init_off = encode(&canonical(&parse(INIT).unwrap()));
    let init_e = vals[init_off as usize];
    println!("=== dense solve (unfolded cohort index) ===");
    println!("slots         {n}  (~{} MB)", n / (1 << 20));
    println!(
        "initial value e = {init_e}  ({} plies)",
        signed_plies(init_e)
    );

    // census + max DTM over real positions, in parallel
    let (w, l, d, max_plies) = (0..POSITION_TOTAL_COUNT)
        .into_par_iter()
        .filter(|&off| is_valid_offset(off))
        .map(|off| {
            let e = vals[off as usize];
            let p = signed_plies(e).abs();
            if e > 0 {
                (1u64, 0u64, 0u64, p)
            } else if e < 0 {
                (0, 1, 0, p)
            } else {
                (0, 0, 1, 0)
            }
        })
        .reduce(
            || (0, 0, 0, 0),
            |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3.max(b.3)),
        );
    let total = w + l + d;
    println!("positions     {total}");
    println!("win {w}  loss {l}  draw {d}");
    println!("max DTM       {max_plies} plies");

    // ---- clausecker spot-check ----
    clausecker_spotcheck(&vals);
}

fn clausecker_spotcheck(vals: &[i8]) {
    let probe_bin = "../external/clausecker-dobutsu/probe";
    let tb = "../external/clausecker-dobutsu/dobutsu.tb";
    let Ok(mut child) = Command::new(probe_bin)
        .arg(tb)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    else {
        println!("clausecker spot-check: probe not available, skipped");
        return;
    };
    let mut cin = child.stdin.take().unwrap();
    let mut cout = BufReader::new(child.stdout.take().unwrap());
    let (mut checked, mut mismatch, mut skipped) = (0u64, 0u64, 0u64);

    let step = (POSITION_TOTAL_COUNT / 5000).max(1);
    let mut off = 0u64;
    while off < POSITION_TOTAL_COUNT {
        if let Some(p) = decode_checked(off) {
            writeln!(cin, "{}", format(&p)).unwrap();
            cin.flush().unwrap();
            let mut resp = String::new();
            cout.read_line(&mut resp).unwrap();
            if resp.contains("\"error\"") {
                skipped += 1;
            } else {
                let res = resp
                    .split("\"result\":\"")
                    .nth(1)
                    .and_then(|s| s.split('"').next())
                    .unwrap_or("");
                let dtm: i32 = resp
                    .split("\"dtm\":")
                    .nth(1)
                    .and_then(|s| s.split(|c: char| !c.is_ascii_digit() && c != '-').next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let theirs = match res {
                    "win" => dtm,
                    "loss" => -dtm,
                    _ => 0,
                };
                if theirs != signed_plies(vals[off as usize]) {
                    mismatch += 1;
                    if mismatch <= 5 {
                        eprintln!(
                            "mismatch at {off}: ours {} plies, theirs {theirs}\n  {}",
                            signed_plies(vals[off as usize]),
                            format(&p)
                        );
                    }
                }
                checked += 1;
            }
        }
        off += step;
    }
    println!("clausecker spot-check: checked {checked}, mismatches {mismatch}, skipped {skipped}");
}
