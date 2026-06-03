//! Serve our own tablebase the way the clausecker probe does: load the
//! `--save`d file (sorted u64-key / i16-value records), then for each position
//! string read on stdin print one JSON line with its value and every legal
//! move's result. Same protocol as tools/probe.c, so explorer/serve.py can use
//! either backend interchangeably.
//!
//!   tbprobe dobutsu.tb.bin

use std::env;
use std::io::{self, BufRead, Write};

use dobutsu::{canonical_key, format, notation, parse, Owner};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).map(String::as_str).unwrap_or("dobutsu.tb.bin");
    let data = std::fs::read(path).expect("read tablebase file");
    let n = data.len() / 10;
    let mut tbkeys: Vec<u64> = Vec::with_capacity(n);
    let mut tbvals: Vec<i16> = Vec::with_capacity(n);
    for i in 0..n {
        let o = i * 10;
        tbkeys.push(u64::from_le_bytes(data[o..o + 8].try_into().unwrap()));
        tbvals.push(i16::from_le_bytes(data[o + 8..o + 10].try_into().unwrap()));
    }
    drop(data);
    let lookup = |k: u64| tbkeys.binary_search(&k).ok().map(|i| tbvals[i]);

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut line = String::new();
    loop {
        line.clear();
        if reader.read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }
        let s = line.trim();
        let p = match parse(s) {
            Some(p) => p,
            None => {
                writeln!(out, "{{\"error\":\"bad position\"}}").unwrap();
                out.flush().unwrap();
                continue;
            }
        };
        let val = match lookup(canonical_key(&p)) {
            Some(v) => v,
            None => {
                writeln!(out, "{{\"error\":\"not in tablebase\"}}").unwrap();
                out.flush().unwrap();
                continue;
            }
        };
        let (vres, vdtm) = classify(val);
        let side = if p.turn == Owner::Sente { 'S' } else { 'G' };
        let mut buf = format!(
            "{{\"pos\":\"{s}\",\"side\":\"{side}\",\"value\":{{\"result\":\"{vres}\",\"dtm\":{vdtm}}},\"moves\":["
        );
        let mut first = true;
        for m in p.moves() {
            let (res, dtm, to): (&str, i64, String);
            if p.is_terminal_win_move(&m) {
                res = "win";
                dtm = 1;
                to = String::new();
            } else {
                let child = p.make(&m);
                match lookup(canonical_key(&child)) {
                    None => continue, // child outside the solved set (only at terminal positions)
                    Some(cv) => {
                        if cv < 0 {
                            res = "win";
                            dtm = -(cv as i64) + 1;
                        } else if cv > 0 {
                            res = "loss";
                            dtm = cv as i64 + 1;
                        } else {
                            res = "draw";
                            dtm = 0;
                        }
                        to = format(&child);
                    }
                }
            }
            if !first {
                buf.push(',');
            }
            first = false;
            buf.push_str(&format!(
                "{{\"move\":\"{}\",\"result\":\"{res}\",\"dtm\":{dtm},\"to\":\"{to}\"}}",
                notation(&m)
            ));
        }
        buf.push_str("]}");
        writeln!(out, "{buf}").unwrap();
        out.flush().unwrap();
    }
}

fn classify(v: i16) -> (&'static str, i64) {
    if v > 0 {
        ("win", v as i64)
    } else if v < 0 {
        ("loss", -(v as i64))
    } else {
        ("draw", 0)
    }
}
