//! A computed combinatorial index (rank/unrank) over turn-canonical positions
//! (Sente to move). It is a bijection onto `[0, count)`, so the solve can use a
//! flat array instead of a hash map keyed by the packed position. Folds turn
//! (via `canonical`) but not the left-right mirror (a later refinement).
//!
//! `rank = lion_pair_index * SIZE + within`. `lion_pair_index` in `[0, 132)`
//! encodes (Sente lion square, Gote lion square != Sente). `SIZE` is the number
//! of ways to fill the 10 non-lion cells (and the hands) with the 2 giraffes, 2
//! elephants, and 2 chick/hens. `within` is a mixed-radix rank computed
//! cell-by-cell with a DP `ways[cells][g][e][c]` = number of completions.

use crate::{Owner, Piece, Position};

pub struct Ranker {
    ways: [[[[u64; 3]; 3]; 3]; 11], // ways[cells_remaining][giraffes][elephants][chicks]
    pub count: u64,
}

impl Default for Ranker {
    fn default() -> Self {
        Self::new()
    }
}

impl Ranker {
    pub fn new() -> Self {
        let mut ways = [[[[0u64; 3]; 3]; 3]; 11];
        // base case: no cells left — the leftover pieces split between the two
        // hands, (g+1)(e+1)(c+1) ways.
        for g in 0..3 {
            for e in 0..3 {
                for c in 0..3 {
                    ways[0][g][e][c] = (g as u64 + 1) * (e as u64 + 1) * (c as u64 + 1);
                }
            }
        }
        for n in 1..11 {
            for g in 0..3 {
                for e in 0..3 {
                    for c in 0..3 {
                        let mut w = ways[n - 1][g][e][c]; // this cell empty
                        if g > 0 {
                            w += 2 * ways[n - 1][g - 1][e][c]; // giraffe, two owners
                        }
                        if e > 0 {
                            w += 2 * ways[n - 1][g][e - 1][c]; // elephant, two owners
                        }
                        if c > 0 {
                            w += 4 * ways[n - 1][g][e][c - 1]; // chick or hen, two owners
                        }
                        ways[n][g][e][c] = w;
                    }
                }
            }
        }
        let count = 132 * ways[10][2][2][2];
        Ranker { ways, count }
    }

    pub fn rank(&self, p: &Position) -> u64 {
        let (mut sl, mut gl) = (12usize, 12usize);
        for (s, cell) in p.board.iter().enumerate() {
            if let Some((Piece::Lion, o)) = cell {
                if *o == Owner::Sente {
                    sl = s;
                } else {
                    gl = s;
                }
            }
        }
        debug_assert!(
            sl < 12 && gl < 12 && sl != gl,
            "rank requires both lions on the board"
        );
        let lion_idx = (sl * 11 + if gl < sl { gl } else { gl - 1 }) as u64;

        let (mut g, mut e, mut c) = (2usize, 2usize, 2usize);
        let mut within = 0u64;
        let mut after = 9usize; // cells remaining after the current one
        for s in 0..12 {
            if s == sl || s == gl {
                continue;
            }
            let cc = content_code(p.board[s]);
            for code in 0..cc {
                if let Some((g2, e2, c2)) = apply(code, g, e, c) {
                    within += self.ways[after][g2][e2][c2];
                }
            }
            let (g2, e2, c2) = apply(cc, g, e, c).expect("budget exhausted");
            g = g2;
            e = e2;
            c = c2;
            after = after.wrapping_sub(1);
        }
        // leftover pieces split between the hands
        let hs = p.hand_sente;
        let h_idx = hs[0] as u64 + (g as u64 + 1) * (hs[1] as u64 + (e as u64 + 1) * hs[2] as u64);
        within += h_idx;

        lion_idx * self.ways[10][2][2][2] + within
    }

    pub fn unrank(&self, i: u64) -> Position {
        let size = self.ways[10][2][2][2];
        let lion_idx = (i / size) as usize;
        let mut within = i % size;
        let sl = lion_idx / 11;
        let r = lion_idx % 11;
        let gl = if r < sl { r } else { r + 1 };

        let mut board: [Option<(Piece, Owner)>; 12] = [None; 12];
        board[sl] = Some((Piece::Lion, Owner::Sente));
        board[gl] = Some((Piece::Lion, Owner::Gote));

        let (mut g, mut e, mut c) = (2usize, 2usize, 2usize);
        let mut after = 9usize;
        for s in 0..12 {
            if s == sl || s == gl {
                continue;
            }
            let mut chosen = 0u8;
            for code in 0..9u8 {
                if let Some((g2, e2, c2)) = apply(code, g, e, c) {
                    let w = self.ways[after][g2][e2][c2];
                    if within < w {
                        chosen = code;
                        g = g2;
                        e = e2;
                        c = c2;
                        break;
                    }
                    within -= w;
                }
            }
            board[s] = piece_for_code(chosen);
            after = after.wrapping_sub(1);
        }
        let hs0 = (within % (g as u64 + 1)) as u8;
        within /= g as u64 + 1;
        let hs1 = (within % (e as u64 + 1)) as u8;
        within /= e as u64 + 1;
        let hs2 = within as u8;
        Position {
            board,
            hand_sente: [hs0, hs1, hs2],
            hand_gote: [g as u8 - hs0, e as u8 - hs1, c as u8 - hs2],
            turn: Owner::Sente,
        }
    }
}

fn content_code(cell: Option<(Piece, Owner)>) -> u8 {
    match cell {
        None => 0,
        Some((Piece::Giraffe, Owner::Sente)) => 1,
        Some((Piece::Giraffe, Owner::Gote)) => 2,
        Some((Piece::Elephant, Owner::Sente)) => 3,
        Some((Piece::Elephant, Owner::Gote)) => 4,
        Some((Piece::Chick, Owner::Sente)) => 5,
        Some((Piece::Chick, Owner::Gote)) => 6,
        Some((Piece::Hen, Owner::Sente)) => 7,
        Some((Piece::Hen, Owner::Gote)) => 8,
        Some((Piece::Lion, _)) => unreachable!("lions are handled separately"),
    }
}

fn piece_for_code(code: u8) -> Option<(Piece, Owner)> {
    match code {
        0 => None,
        1 => Some((Piece::Giraffe, Owner::Sente)),
        2 => Some((Piece::Giraffe, Owner::Gote)),
        3 => Some((Piece::Elephant, Owner::Sente)),
        4 => Some((Piece::Elephant, Owner::Gote)),
        5 => Some((Piece::Chick, Owner::Sente)),
        6 => Some((Piece::Chick, Owner::Gote)),
        7 => Some((Piece::Hen, Owner::Sente)),
        8 => Some((Piece::Hen, Owner::Gote)),
        _ => unreachable!(),
    }
}

fn apply(code: u8, g: usize, e: usize, c: usize) -> Option<(usize, usize, usize)> {
    match code {
        0 => Some((g, e, c)),
        1 | 2 => (g > 0).then_some((g - 1, e, c)),
        3 | 4 => (e > 0).then_some((g, e - 1, c)),
        5 | 6 | 7 | 8 => (c > 0).then_some((g, e, c - 1)),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{canonical, pack, parse};
    use std::collections::{HashSet, VecDeque};

    const INIT: &str = "S/gle/-c-/-C-/ELG/-";

    #[test]
    fn roundtrips_on_reachable_sample() {
        let r = Ranker::new();
        assert!(r.count > 0);
        let init = canonical(&parse(INIT).unwrap());

        let mut seen: HashSet<u64> = HashSet::new();
        let mut ranks: HashSet<u64> = HashSet::new();
        let mut q: VecDeque<Position> = VecDeque::new();
        seen.insert(pack(&init));
        q.push_back(init);

        let mut n = 0u64;
        while let Some(p) = q.pop_front() {
            let idx = r.rank(&p);
            assert!(idx < r.count, "rank {idx} >= count {}", r.count);
            assert!(ranks.insert(idx), "rank collision at {idx}");
            assert_eq!(r.unrank(idx), p, "unrank mismatch");
            n += 1;
            if n >= 1_000_000 {
                break;
            }
            let ms = p.moves();
            if ms.iter().any(|m| p.is_terminal_win_move(m)) {
                continue;
            }
            for m in &ms {
                let child = canonical(&p.make(m));
                if seen.insert(pack(&child)) {
                    q.push_back(child);
                }
            }
        }
        assert!(n > 100_000, "sample too small: {n}");
    }
}
