//! Validate the rules engine against clausecker across many positions.
//!
//! BFS from the initial position; for each position, compare our move-set to the
//! clausecker probe's. Children are canonicalized through our own parser so that
//! hand-ordering differences don't register as mismatches; lion-capture moves
//! (which end the game) are counted but excluded from the child set.
//!
//!   cargo run --release --bin oracle_diff [PROBE_BIN] [TABLEBASE]
//! (defaults assume it's run from solver/ with the clausecker checkout in ../external)

use std::collections::{HashSet, VecDeque};
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use dobutsu::{format, parse, Owner, Piece, Position};

const INIT: &str = "S/gle/-c-/-C-/ELG/-";
const LIMIT: usize = 200_000;

/// clausecker's position_valid rejects positions with the two lions adjacent or
/// a lion already on the enemy back rank ("ascended"). These are resolution
/// positions (a lion is capturable or the game is won); they're still valid
/// children of their parents, so we validate the move *into* them but don't
/// re-probe them as standalone positions.
fn clausecker_invalid(p: &Position) -> bool {
    let (mut sl, mut gl) = (255i32, 255i32);
    for sq in 0..12i32 {
        match p.board[sq as usize] {
            Some((Piece::Lion, Owner::Sente)) => sl = sq,
            Some((Piece::Lion, Owner::Gote)) => gl = sq,
            _ => {}
        }
    }
    if sl / 3 == 0 || gl / 3 == 3 {
        return true; // a lion has ascended to the enemy back rank
    }
    let (dr, df) = (((sl / 3) - (gl / 3)).abs(), ((sl % 3) - (gl % 3)).abs());
    dr <= 1 && df <= 1 // lions adjacent
}

/// pull every `"to":"..."` value out of a probe JSON line
fn extract_tos(json: &str) -> Vec<String> {
    let key = "\"to\":\"";
    let mut v = Vec::new();
    let mut i = 0;
    while let Some(p) = json[i..].find(key) {
        let s = i + p + key.len();
        match json[s..].find('"') {
            Some(e) => {
                v.push(json[s..s + e].to_string());
                i = s + e + 1;
            }
            None => break,
        }
    }
    v
}

fn canon(posstr: &str) -> Option<String> {
    parse(posstr).map(|p| format(&p))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let probe_bin = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("../external/clausecker-dobutsu/probe");
    let tb = args
        .get(2)
        .map(String::as_str)
        .unwrap_or("../external/clausecker-dobutsu/dobutsu.tb");

    let mut child = Command::new(probe_bin)
        .arg(tb)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn probe (build it first; see explorer/README.md)");
    let mut cin = child.stdin.take().unwrap();
    let mut cout = BufReader::new(child.stdout.take().unwrap());

    let init = parse(INIT).unwrap();
    let mut seen: HashSet<String> = HashSet::new();
    let mut q: VecDeque<Position> = VecDeque::new();
    seen.insert(format(&init));
    q.push_back(init);

    let mut checked = 0usize;
    let mut mismatches = 0usize;

    while let Some(p) = q.pop_front() {
        if checked >= LIMIT {
            break;
        }

        let mut our: HashSet<String> = HashSet::new();
        let mut our_terminal = 0usize;
        let mut our_total = 0usize;
        for m in p.moves() {
            our_total += 1;
            if p.is_terminal_win_move(&m) {
                our_terminal += 1;
                continue;
            }
            let cp = p.make(&m);
            let f = format(&cp);
            our.insert(f.clone());
            // children clausecker won't parse standalone are still valid children
            // (kept in `our` for the parent comparison) but not enqueued/re-probed
            if !clausecker_invalid(&cp) && seen.insert(f) {
                q.push_back(cp);
            }
        }

        writeln!(cin, "{}", format(&p)).unwrap();
        cin.flush().unwrap();
        let mut resp = String::new();
        cout.read_line(&mut resp).unwrap();
        let tos = extract_tos(&resp);
        let ckr_total = tos.len();
        let mut ckr: HashSet<String> = HashSet::new();
        let mut ckr_lioncap = 0usize;
        for t in &tos {
            if t.is_empty() {
                ckr_lioncap += 1;
            } else if let Some(c) = canon(t) {
                ckr.insert(c);
            }
        }

        if our != ckr || our_total != ckr_total || our_terminal != ckr_lioncap {
            mismatches += 1;
            eprintln!("MISMATCH at {}", format(&p));
            eprintln!("  totals ours={our_total} ckr={ckr_total}  lioncap ours={our_terminal} ckr={ckr_lioncap}");
            let only_ours: Vec<_> = our.difference(&ckr).collect();
            let only_ckr: Vec<_> = ckr.difference(&our).collect();
            if !only_ours.is_empty() {
                eprintln!("  only ours: {only_ours:?}");
            }
            if !only_ckr.is_empty() {
                eprintln!("  only ckr:  {only_ckr:?}");
            }
            if mismatches >= 5 {
                eprintln!("stopping after 5 mismatches");
                break;
            }
        }
        checked += 1;
    }

    println!(
        "checked {checked} positions, {mismatches} mismatch(es); {} distinct seen",
        seen.len()
    );
}
