//! Build the compact tablebase. Reads the `(u64 key, i16 value)` records that
//! `solve --save` writes and emits `dobutsu.ctb`: a minimal perfect hash over
//! the canonical keys (no keys stored) plus the distance-to-mate values packed
//! to 9 bits each. ~6.7x smaller than the sorted-records file, and the file the
//! hosted probe loads.
//!
//!   compact dobutsu.tb.bin dobutsu.ctb
//!
//! Layout: MAGIC(8) | n:u64 | value_bits:u32 | mph_len:u64 | mph(bincode) | packed values

use std::env;
use std::io::Write;

use boomphf::Mphf;

pub const MAGIC: &[u8; 8] = b"DOBCTB01";
pub const BIAS: i32 = 173; // value v in [-173,173] -> code v+BIAS in [0,346], fits 9 bits
pub const VALUE_BITS: u32 = 9;

fn main() {
    let args: Vec<String> = env::args().collect();
    let inp = args.get(1).map(String::as_str).unwrap_or("dobutsu.tb.bin");
    let outp = args.get(2).map(String::as_str).unwrap_or("dobutsu.ctb");
    let t0 = std::time::Instant::now();

    let data = std::fs::read(inp).expect("read tablebase records");
    let n = data.len() / 10;
    let mut keys: Vec<u64> = Vec::with_capacity(n);
    let mut vals: Vec<i16> = Vec::with_capacity(n);
    for i in 0..n {
        let o = i * 10;
        keys.push(u64::from_le_bytes(data[o..o + 8].try_into().unwrap()));
        vals.push(i16::from_le_bytes(data[o + 8..o + 10].try_into().unwrap()));
    }
    drop(data);
    eprintln!("[{:?}] loaded {n} records", t0.elapsed());

    let mph = Mphf::new_parallel(1.7, &keys, None);
    eprintln!("[{:?}] built MPH", t0.elapsed());

    // value codes in MPH-slot order
    let mut codes = vec![0u16; n];
    for i in 0..n {
        let slot = mph.hash(&keys[i]) as usize;
        let code = (vals[i] as i32 + BIAS) as u16;
        assert!(code < (1 << VALUE_BITS), "value {} out of range", vals[i]);
        codes[slot] = code;
    }

    // pack VALUE_BITS-bit codes, little-endian within each code
    let mut packed = vec![0u8; (n * VALUE_BITS as usize + 7) / 8];
    for (i, &c) in codes.iter().enumerate() {
        let base = i * VALUE_BITS as usize;
        for k in 0..VALUE_BITS as usize {
            if (c >> k) & 1 == 1 {
                packed[(base + k) / 8] |= 1 << ((base + k) % 8);
            }
        }
    }
    eprintln!("[{:?}] packed values", t0.elapsed());

    let mph_bytes = bincode::serialize(&mph).expect("serialize mph");

    let f = std::fs::File::create(outp).expect("create output");
    let mut w = std::io::BufWriter::new(f);
    w.write_all(MAGIC).unwrap();
    w.write_all(&(n as u64).to_le_bytes()).unwrap();
    w.write_all(&VALUE_BITS.to_le_bytes()).unwrap();
    w.write_all(&(mph_bytes.len() as u64).to_le_bytes()).unwrap();
    w.write_all(&mph_bytes).unwrap();
    w.write_all(&packed).unwrap();
    w.flush().unwrap();

    let total = 8 + 8 + 4 + 8 + mph_bytes.len() + packed.len();
    eprintln!(
        "[{:?}] wrote {outp}: {:.0} MB total (mph {:.0} MB, values {:.0} MB)",
        t0.elapsed(),
        total as f64 / 1e6,
        mph_bytes.len() as f64 / 1e6,
        packed.len() as f64 / 1e6
    );
}
