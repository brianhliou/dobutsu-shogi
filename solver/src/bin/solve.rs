//! Solve the game: enumerate every reachable (canonical) position, then fill its
//! distance-to-mate by retrograde analysis. Values are from the side-to-move's
//! view: +d = win in d plies, -d = loss in d plies, 0 = draw. The standard game's
//! initial position comes out -78 (Gote wins in 78). With `--no-drops`, solves the
//! variant where captured pieces leave the game (the §4 ablation).
//!
//!   cargo run --release --bin solve              # standard, spot-checks vs clausecker
//!   cargo run --release --bin solve -- --no-drops

use std::collections::{HashMap, VecDeque};
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use dobutsu::{canonical_key, format, parse, unpack, Move, Position};

const INIT: &str = "S/gle/-c-/-C-/ELG/-";
const UNKNOWN: i16 = 0;

fn main() {
    let args: Vec<String> = env::args().collect();
    let no_drops = args.iter().any(|a| a == "--no-drops");
    let save_path = args.iter().position(|a| a == "--save").and_then(|i| args.get(i + 1)).cloned();
    let gen = |p: &Position| -> Vec<Move> { if no_drops { p.moves_nd() } else { p.moves() } };
    let mk = |p: &Position, m: &Move| -> Position { if no_drops { p.make_nd(m) } else { p.make(m) } };
    let is_terminal = |p: &Position, ms: &[Move]| ms.iter().any(|m| p.is_terminal_win_move(m));

    let t0 = std::time::Instant::now();
    eprintln!("variant: {}", if no_drops { "NO-DROPS" } else { "standard" });

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
        let ms = gen(&p);
        if is_terminal(&p, &ms) {
            continue;
        }
        for m in &ms {
            let ck = canonical_key(&mk(&p, m));
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
        let ms = gen(&p);
        if is_terminal(&p, &ms) {
            values[id] = 1;
        } else if ms.is_empty() {
            values[id] = -2; // no legal move = loss
            no_move += 1;
        } else {
            unknown.push(id as u32);
        }
    }
    eprintln!("[{:?}] terminal-win {}, no-move {no_move}, unknown {}",
        t0.elapsed(), n as u64 - unknown.len() as u64 - no_move, unknown.len());

    // ---- retrograde fixpoint (Jacobi: decide on prior-round values) ----
    let mut round = 0;
    loop {
        round += 1;
        let mut decisions: Vec<(u32, i16)> = Vec::new();
        let mut next: Vec<u32> = Vec::with_capacity(unknown.len() / 2);
        for &id in &unknown {
            let p = unpack(keys[id as usize]);
            let mut best_win: Option<i16> = None;
            let mut worst_loss: i16 = 0;
            let mut any_unknown = false;
            for m in &gen(&p) {
                let v = values[*index.get(&canonical_key(&mk(&p, m))).unwrap() as usize];
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
        if round % 10 == 1 || decided == 0 {
            eprintln!("[{:?}] round {round}: decided {decided}, remaining {}", t0.elapsed(), unknown.len());
        }
        if decided == 0 {
            break;
        }
    }
    let draws = unknown.len();
    drop(index);

    // ---- serialize: sorted (u64 key, i16 value) records for binary-search probing ----
    if let Some(path) = &save_path {
        let mut order: Vec<u32> = (0..n as u32).collect();
        order.sort_unstable_by_key(|&i| keys[i as usize]);
        let f = std::fs::File::create(path).expect("create tablebase file");
        let mut wf = std::io::BufWriter::new(f);
        for &i in &order {
            wf.write_all(&keys[i as usize].to_le_bytes()).unwrap();
            wf.write_all(&values[i as usize].to_le_bytes()).unwrap();
        }
        wf.flush().unwrap();
        eprintln!("[{:?}] wrote {n} entries to {path}", t0.elapsed());
    }

    // ---- report ----
    let (mut w, mut l, mut d) = (0u64, 0u64, 0u64);
    let mut maxdtm = 0i16;
    for &v in &values {
        if v > 0 { w += 1; maxdtm = maxdtm.max(v); } else if v < 0 { l += 1; } else { d += 1; }
    }
    println!("=== variant: {} ===", if no_drops { "NO-DROPS" } else { "standard" });
    println!("positions     {n}");
    println!("initial value {}", values[0]);
    println!("win {w}  loss {l}  draw {d}  (unresolved->draw {draws})");
    println!("draw rate     {:.2}%", 100.0 * d as f64 / n as f64);
    println!("max DTM       {maxdtm} plies");

    // ---- clausecker spot-check (standard variant only) ----
    if !no_drops {
        // Positional args = non-flag tokens, excluding the path that follows
        // --save (else `solve --save out.bin` would feed our own file to the
        // clausecker probe as its tablebase and every lookup would mismatch).
        let save_idx = args.iter().position(|a| a == "--save");
        let positional: Vec<&str> = args.iter().enumerate()
            .filter(|(i, a)| *i != 0 && !a.starts_with("--")
                && save_idx.map_or(true, |si| *i != si + 1))
            .map(|(_, a)| a.as_str())
            .collect();
        let probe_bin = positional.first().copied()
            .unwrap_or("../external/clausecker-dobutsu/probe");
        let tb = positional.get(1).copied()
            .unwrap_or("../external/clausecker-dobutsu/dobutsu.tb");
        if let Ok(mut child) = Command::new(probe_bin).arg(tb)
            .stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()
        {
            let mut cin = child.stdin.take().unwrap();
            let mut cout = BufReader::new(child.stdout.take().unwrap());
            let (mut checked, mut mismatch, mut skipped) = (0u64, 0u64, 0u64);
            let mut i = 0;
            let step = (n / 5000).max(1);
            while i < n {
                writeln!(cin, "{}", format(&unpack(keys[i]))).unwrap();
                cin.flush().unwrap();
                let mut resp = String::new();
                cout.read_line(&mut resp).unwrap();
                if resp.contains("\"error\"") {
                    skipped += 1;
                } else {
                    let res = resp.split("\"result\":\"").nth(1).and_then(|s| s.split('"').next()).unwrap_or("");
                    let dtm: i64 = resp.split("\"dtm\":").nth(1)
                        .and_then(|s| s.split(|c: char| !c.is_ascii_digit() && c != '-').next())
                        .and_then(|s| s.parse().ok()).unwrap_or(0);
                    let theirs: i64 = match res { "win" => dtm, "loss" => -dtm, _ => 0 };
                    if theirs != values[i] as i64 {
                        mismatch += 1;
                    }
                    checked += 1;
                }
                i += step;
            }
            println!("clausecker spot-check: checked {checked}, mismatches {mismatch}, skipped {skipped}");
        }
    }
}
