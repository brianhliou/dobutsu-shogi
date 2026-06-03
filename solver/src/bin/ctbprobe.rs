//! Serve the compact tablebase (`dobutsu.ctb`, built by `compact`). Same
//! stdin/stdout JSON protocol as `tbprobe`/`tools/probe.c`, so `explorer/serve.py`
//! uses it interchangeably — it just loads ~320 MB instead of 2.14 GB.
//!
//!   ctbprobe dobutsu.ctb
//!
//! The minimal perfect hash has no notion of membership, so it must never be
//! asked about a position outside the solved set. Two guards keep that true:
//!   - the queried position is looked up with `try_hash` (returns None, not a
//!     panic, on an out-of-set position typed into a URL);
//!   - at a terminal position (a lion capture or safe Try is available) only the
//!     winning move is emitted — its non-winning siblings lead to positions we
//!     never solved (terminal nodes aren't expanded), so we don't look them up.

use std::env;
use std::io::{self, BufRead, Read, Write};

use boomphf::Mphf;
use dobutsu::{canonical_key, format, notation, parse, Owner};

const MAGIC: &[u8; 8] = b"DOBCTB01";
const BIAS: i32 = 173;

struct Tb {
    mph: Mphf<u64>,
    packed: Vec<u8>,
    bits: usize,
}

impl Tb {
    // Stream the file so we never hold two copies of the 240 MB value region:
    // read the header, then the MPH bytes (dropped after deserialize), then the
    // packed values straight into their final Vec.
    fn load(path: &str) -> Tb {
        let mut f = std::fs::File::open(path).expect("open compact tablebase");
        let mut hdr = [0u8; 28];
        f.read_exact(&mut hdr).expect("read header");
        assert_eq!(&hdr[0..8], MAGIC, "not a dobutsu.ctb file");
        let bits = u32::from_le_bytes(hdr[16..20].try_into().unwrap()) as usize;
        let mph_len = u64::from_le_bytes(hdr[20..28].try_into().unwrap()) as usize;
        let mut mph_buf = vec![0u8; mph_len];
        f.read_exact(&mut mph_buf).expect("read mph");
        let mph: Mphf<u64> = bincode::deserialize(&mph_buf).expect("deserialize mph");
        drop(mph_buf);
        let mut packed = Vec::new();
        f.read_to_end(&mut packed).expect("read values");
        Tb { mph, packed, bits }
    }

    /// Value of a position, or None if it is not in the solved set.
    fn value(&self, key: u64) -> Option<i16> {
        let slot = self.mph.try_hash(&key)? as usize;
        let base = slot * self.bits;
        let mut code = 0u16;
        for k in 0..self.bits {
            if self.packed[(base + k) / 8] >> ((base + k) % 8) & 1 == 1 {
                code |= 1 << k;
            }
        }
        Some(code as i32 as i16 - BIAS as i16)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).map(String::as_str).unwrap_or("dobutsu.ctb");
    let tb = Tb::load(path);

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
        let val = match tb.value(canonical_key(&p)) {
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
        let moves = p.moves();
        let terminal = moves.iter().any(|m| p.is_terminal_win_move(m));
        let mut first = true;
        for m in &moves {
            let (res, dtm, to): (&str, i64, String);
            if p.is_terminal_win_move(m) {
                res = "win";
                dtm = 1;
                to = String::new();
            } else if terminal {
                continue; // non-winning sibling of a terminal move: child unsolved
            } else {
                let child = p.make(m);
                match tb.value(canonical_key(&child)) {
                    None => continue,
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
                notation(m)
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
