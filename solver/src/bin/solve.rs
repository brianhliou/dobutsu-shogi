//! Solve the game: enumerate every reachable (canonical) position, then fill its
//! distance-to-mate by retrograde analysis. Values are from the side-to-move's
//! view: +d = win in d plies, -d = loss in d plies, 0 = draw. The initial
//! position should come out -78 (Sente to move, loses in 78 = Gote wins in 78).
//! Spot-checks a sample against clausecker's probe.
//!
//!   cargo run --release --bin solve [PROBE_BIN] [TABLEBASE]

use std::collections::{HashMap, VecDeque};
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use dobutsu::{canonical_key, format, parse, unpack, Position};

const INIT: &str = "S/gle/-c-/-C-/ELG/-";
const UNKNOWN: i16 = 0;

fn terminal_win(p: &Position, ms: &[dobutsu::Move]) -> bool {
    ms.iter().any(|m| p.is_terminal_win_move(m))
}

fn main() {
    let t0 = std::time::Instant::now();

    // ---- enumerate canonical reachable positions, assigning dense ids ----
    let init_key = canonical_key(&parse(INIT).unwrap());
    let mut index: HashMap<u64, u32> = HashMap::new();
    let mut keys: Vec<u64> = Vec::new();
    let mut q: VecDeque<u64> = VecDeque::new();
    index.insert(init_key, 0);
    keys.push(init_key);
    q.push_back(init_key);
    while let Some(k) = q.pop_front() {
        let p = unpack(k);
        let ms = p.moves();
        if terminal_win(&p, &ms) {
            continue; // terminal: not expanded
        }
        for m in &ms {
            let ck = canonical_key(&p.make(m));
            if !index.contains_key(&ck) {
                index.insert(ck, keys.len() as u32);
                keys.push(ck);
                q.push_back(ck);
            }
        }
    }
    drop(q);
    let n = keys.len();
    eprintln!("[{:?}] enumerated {n} positions", t0.elapsed());

    // ---- initialize: terminal wins = +1, others unknown ----
    let mut values = vec![UNKNOWN; n];
    let mut unknown: Vec<u32> = Vec::new();
    let mut no_move = 0u64;
    for id in 0..n {
        let p = unpack(keys[id]);
        let ms = p.moves();
        if terminal_win(&p, &ms) {
            values[id] = 1;
        } else if ms.is_empty() {
            values[id] = -2; // no legal move = loss (should be unreachable)
            no_move += 1;
        } else {
            unknown.push(id as u32);
        }
    }
    eprintln!("[{:?}] terminal wins {}, no-move {no_move}, unknown {}",
        t0.elapsed(), n as u64 - unknown.len() as u64 - no_move, unknown.len());

    // ---- retrograde fixpoint (Jacobi: decide on prior-round values only) ----
    let mut round = 0;
    loop {
        round += 1;
        let mut decisions: Vec<(u32, i16)> = Vec::new();
        let mut next: Vec<u32> = Vec::with_capacity(unknown.len() / 2);
        for &id in &unknown {
            let p = unpack(keys[id as usize]);
            let mut best_win: Option<i16> = None; // fastest losing successor -> our win
            let mut worst_loss: i16 = 0; // slowest winning successor -> our loss delay
            let mut any_unknown = false;
            for m in &p.moves() {
                let v = values[*index.get(&canonical_key(&p.make(m))).unwrap() as usize];
                if v == UNKNOWN {
                    any_unknown = true;
                } else if v < 0 {
                    let d = -v;
                    best_win = Some(best_win.map_or(d, |b| b.min(d)));
                } else {
                    worst_loss = worst_loss.max(v);
                }
            }
            if let Some(d) = best_win {
                decisions.push((id, d + 1));
            } else if any_unknown {
                next.push(id);
            } else {
                decisions.push((id, -(worst_loss + 1)));
            }
        }
        let decided = decisions.len();
        for (id, v) in &decisions {
            values[*id as usize] = *v;
        }
        unknown = next;
        eprintln!("[{:?}] round {round}: decided {decided}, remaining {}", t0.elapsed(), unknown.len());
        if decided == 0 {
            break;
        }
    }
    let draws = unknown.len();

    // ---- report ----
    let (mut w, mut l, mut d) = (0u64, 0u64, 0u64);
    let mut maxdtm = 0i16;
    for &v in &values {
        if v > 0 { w += 1; maxdtm = maxdtm.max(v); } else if v < 0 { l += 1; } else { d += 1; }
    }
    println!("positions {n}");
    println!("initial value: {} (expect -78)", values[0]);
    println!("win {w}  loss {l}  draw {d}  (unresolved->draw {draws})  max-dtm {maxdtm}");

    // ---- spot-check vs clausecker ----
    let args: Vec<String> = env::args().collect();
    let probe_bin = args.get(1).map(String::as_str).unwrap_or("../external/clausecker-dobutsu/probe");
    let tb = args.get(2).map(String::as_str).unwrap_or("../external/clausecker-dobutsu/dobutsu.tb");
    if let Ok(mut child) = Command::new(probe_bin).arg(tb)
        .stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()
    {
        let mut cin = child.stdin.take().unwrap();
        let mut cout = BufReader::new(child.stdout.take().unwrap());
        let (mut checked, mut mismatch, mut skipped) = (0u64, 0u64, 0u64);
        let step = (n / 5000).max(1);
        let mut i = 0;
        while i < n {
            let p = unpack(keys[i]);
            writeln!(cin, "{}", format(&p)).unwrap();
            cin.flush().unwrap();
            let mut resp = String::new();
            cout.read_line(&mut resp).unwrap();
            if resp.contains("\"error\"") {
                skipped += 1;
            } else {
                // parse "result":"X" and "dtm":N from the value object
                let ours = values[i];
                let res = resp.split("\"result\":\"").nth(1).and_then(|s| s.split('"').next()).unwrap_or("");
                let dtm: i64 = resp.split("\"dtm\":").nth(1)
                    .and_then(|s| s.split(|c: char| !c.is_ascii_digit() && c != '-').next())
                    .and_then(|s| s.parse().ok()).unwrap_or(0);
                let theirs: i64 = match res { "win" => dtm, "loss" => -dtm, _ => 0 };
                if theirs != ours as i64 {
                    if mismatch < 10 {
                        eprintln!("MISMATCH {}: ours={ours} clausecker={theirs} ({res} {dtm})", format(&p));
                    }
                    mismatch += 1;
                }
                checked += 1;
            }
            i += step;
        }
        println!("clausecker spot-check: checked {checked}, mismatches {mismatch}, skipped(invalid) {skipped}");
    } else {
        eprintln!("(probe not found; skipped clausecker spot-check)");
    }
}
