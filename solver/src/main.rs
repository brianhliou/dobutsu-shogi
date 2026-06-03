//! CLI for spot-checking the rules engine: read a position string per line on
//! stdin, print each legal move and the resulting position string. Handy for
//! diffing against clausecker's probe.
//!
//!   echo 'S/gle/-c-/-C-/ELG/-' | cargo run --quiet

use std::io::{self, BufRead, Write};

use dobutsu::{format, notation, parse};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }
        match parse(&line) {
            None => writeln!(out, "bad position: {line}").ok(),
            Some(p) => {
                for m in p.moves() {
                    writeln!(out, "{} {}", notation(&m), format(&p.make(&m))).ok();
                }
                writeln!(out).ok()
            }
        };
    }
}
