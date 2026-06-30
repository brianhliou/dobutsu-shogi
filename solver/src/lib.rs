//! Dōbutsu Shōgi rules engine — our own, validated against clausecker/dobutsu.
//!
//! Squares are 0..12 with `square = rank*3 + file`; rank 0 is the top row (the
//! second player's back rank), rank 3 the bottom (the first player's back rank);
//! files a, b, c are 0, 1, 2. The first player (Sente) starts at the bottom and
//! advances toward rank 0. Position strings use clausecker's format
//! `side / rank1 / rank2 / rank3 / rank4 / hand`, e.g. `S/gle/-c-/-C-/ELG/-`.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Owner {
    Sente,
    Gote,
}

impl Owner {
    pub fn flip(self) -> Owner {
        match self {
            Owner::Sente => Owner::Gote,
            Owner::Gote => Owner::Sente,
        }
    }
    /// rank a chick of this owner promotes on (the enemy's back rank)
    fn enemy_back_rank(self) -> i8 {
        match self {
            Owner::Sente => 0,
            Owner::Gote => 3,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Piece {
    Lion,
    Giraffe,
    Elephant,
    Chick,
    Hen,
}

impl Piece {
    fn hand_index(self) -> usize {
        match self {
            Piece::Giraffe => 0,
            Piece::Elephant => 1,
            Piece::Chick => 2,
            _ => panic!("piece cannot be held in hand"),
        }
    }
}

const HAND_PIECES: [Piece; 3] = [Piece::Giraffe, Piece::Elephant, Piece::Chick];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub board: [Option<(Piece, Owner)>; 12],
    pub hand_sente: [u8; 3], // [giraffe, elephant, chick]
    pub hand_gote: [u8; 3],
    pub turn: Owner,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub piece: Piece,     // piece type before any promotion
    pub from: Option<u8>, // None = a drop from hand
    pub to: u8,
    pub capture: bool,
    pub promote: bool,
}

#[inline]
fn rank(sq: u8) -> i8 {
    (sq / 3) as i8
}
#[inline]
fn file(sq: u8) -> i8 {
    (sq % 3) as i8
}

/// step directions (drank, dfile); "forward" is toward the enemy back rank
fn dirs(piece: Piece, owner: Owner) -> &'static [(i8, i8)] {
    match (piece, owner) {
        (Piece::Lion, _) => &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ],
        (Piece::Giraffe, _) => &[(-1, 0), (1, 0), (0, -1), (0, 1)],
        (Piece::Elephant, _) => &[(-1, -1), (-1, 1), (1, -1), (1, 1)],
        (Piece::Chick, Owner::Sente) => &[(-1, 0)],
        (Piece::Chick, Owner::Gote) => &[(1, 0)],
        // hen = gold general: all directions except the two backward diagonals
        (Piece::Hen, Owner::Sente) => &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, 0)],
        (Piece::Hen, Owner::Gote) => &[(1, -1), (1, 0), (1, 1), (0, -1), (0, 1), (-1, 0)],
    }
}

impl Position {
    fn hand(&self, o: Owner) -> &[u8; 3] {
        match o {
            Owner::Sente => &self.hand_sente,
            Owner::Gote => &self.hand_gote,
        }
    }
    fn hand_mut(&mut self, o: Owner) -> &mut [u8; 3] {
        match o {
            Owner::Sente => &mut self.hand_sente,
            Owner::Gote => &mut self.hand_gote,
        }
    }

    /// All pseudo-legal moves for the side to move (board steps + drops).
    pub fn moves(&self) -> Vec<Move> {
        let mut out = Vec::with_capacity(16);
        self.board_moves(&mut out);
        self.drop_moves(&mut out);
        out
    }

    /// Board steps only — the move set of the no-drops variant.
    pub fn moves_nd(&self) -> Vec<Move> {
        let mut out = Vec::with_capacity(12);
        self.board_moves(&mut out);
        out
    }

    fn board_moves(&self, out: &mut Vec<Move>) {
        let me = self.turn;
        for sq in 0..12u8 {
            let (pc, o) = match self.board[sq as usize] {
                Some(x) => x,
                None => continue,
            };
            if o != me {
                continue;
            }
            let (r, f) = (rank(sq), file(sq));
            for &(dr, df) in dirs(pc, me) {
                let (nr, nf) = (r + dr, f + df);
                if nr < 0 || nr > 3 || nf < 0 || nf > 2 {
                    continue;
                }
                let nsq = (nr * 3 + nf) as u8;
                if let Some((_, o2)) = self.board[nsq as usize] {
                    if o2 == me {
                        continue; // own piece blocks
                    }
                }
                let capture = self.board[nsq as usize].is_some();
                let promote = pc == Piece::Chick && nr == me.enemy_back_rank();
                out.push(Move {
                    piece: pc,
                    from: Some(sq),
                    to: nsq,
                    capture,
                    promote,
                });
            }
        }
    }

    fn drop_moves(&self, out: &mut Vec<Move>) {
        let me = self.turn;
        for (i, &pc) in HAND_PIECES.iter().enumerate() {
            if self.hand(me)[i] == 0 {
                continue;
            }
            for nsq in 0..12u8 {
                if self.board[nsq as usize].is_none() {
                    out.push(Move {
                        piece: pc,
                        from: None,
                        to: nsq,
                        capture: false,
                        promote: false,
                    });
                }
            }
        }
    }

    /// Apply a move, returning the resulting position (turn flipped). A captured
    /// piece enters the captor's hand as its base type; a captured Lion does not
    /// (capturing it ends the game).
    pub fn make(&self, mv: &Move) -> Position {
        let mut p = *self;
        let me = self.turn;
        match mv.from {
            Some(sq) => {
                p.board[sq as usize] = None;
                if mv.capture {
                    if let Some((cap, _)) = self.board[mv.to as usize] {
                        if cap != Piece::Lion {
                            let base = if cap == Piece::Hen { Piece::Chick } else { cap };
                            p.hand_mut(me)[base.hand_index()] += 1;
                        }
                    }
                }
                let placed = if mv.promote { Piece::Hen } else { mv.piece };
                p.board[mv.to as usize] = Some((placed, me));
            }
            None => {
                p.hand_mut(me)[mv.piece.hand_index()] -= 1;
                p.board[mv.to as usize] = Some((mv.piece, me));
            }
        }
        p.turn = me.flip();
        p
    }

    /// No-drops variant of `make`: a captured (non-lion) piece leaves the game
    /// rather than entering the hand. Moves are always board steps here.
    pub fn make_nd(&self, m: &Move) -> Position {
        let mut p = *self;
        let me = self.turn;
        let sq = m.from.expect("no-drops move is always a board move");
        p.board[sq as usize] = None;
        let placed = if m.promote { Piece::Hen } else { m.piece };
        p.board[m.to as usize] = Some((placed, me));
        p.turn = me.flip();
        p
    }

    /// Is `sq` attacked by any of `by`'s pieces on the board (one step away)?
    pub fn is_attacked(&self, sq: u8, by: Owner) -> bool {
        let (tr, tf) = (rank(sq), file(sq));
        for psq in 0..12u8 {
            if let Some((pc, o)) = self.board[psq as usize] {
                if o != by {
                    continue;
                }
                let (dr, df) = (tr - rank(psq), tf - file(psq));
                if dirs(pc, by).iter().any(|&(a, b)| a == dr && b == df) {
                    return true;
                }
            }
        }
        false
    }

    /// Does this move win on the spot — capturing the enemy lion, or a (safe)
    /// Try with one's own lion onto the enemy back rank? Such moves have no
    /// successor position; the game ends.
    pub fn is_terminal_win_move(&self, m: &Move) -> bool {
        if m.from.is_none() {
            return false;
        }
        if m.capture {
            if let Some((Piece::Lion, _)) = self.board[m.to as usize] {
                return true;
            }
        }
        // a Try wins only if the lion reaches the enemy back rank on a safe (unattacked) square
        m.piece == Piece::Lion
            && rank(m.to) == self.turn.enemy_back_rank()
            && !self.is_attacked(m.to, self.turn.flip())
    }

    /// The side to move wins immediately (some move captures the lion or Tries safely).
    pub fn is_immediate_win(&self) -> bool {
        self.moves().iter().any(|m| self.is_terminal_win_move(m))
    }

    /// Side to move can capture the enemy lion with this move (Tanaka's
    /// "win-determined"). Unlike `is_terminal_win_move`, a safe Try is NOT
    /// counted here: Tanaka resolves Tries one ply later (his +1-ply convention).
    pub fn captures_lion(&self, m: &Move) -> bool {
        m.capture && matches!(self.board[m.to as usize], Some((Piece::Lion, _)))
    }

    /// The enemy lion sits on the side-to-move's home back rank: Tanaka's
    /// "loss-determined" position, reached one ply after the opponent's safe Try.
    pub fn enemy_lion_on_home_rank(&self) -> bool {
        let home = self.turn.flip().enemy_back_rank();
        for (sq, cell) in self.board.iter().enumerate() {
            if let Some((Piece::Lion, o)) = cell {
                if *o != self.turn && rank(sq as u8) == home {
                    return true;
                }
            }
        }
        false
    }
}

fn piece_char(pc: Piece) -> char {
    match pc {
        Piece::Lion => 'L',
        Piece::Giraffe => 'G',
        Piece::Elephant => 'E',
        Piece::Chick => 'C',
        Piece::Hen => 'R',
    }
}

fn char_piece(c: char) -> Option<(Piece, Owner)> {
    let owner = if c.is_ascii_uppercase() {
        Owner::Sente
    } else {
        Owner::Gote
    };
    let pc = match c.to_ascii_uppercase() {
        'L' => Piece::Lion,
        'G' => Piece::Giraffe,
        'E' => Piece::Elephant,
        'C' => Piece::Chick,
        'R' => Piece::Hen,
        _ => return None,
    };
    Some((pc, owner))
}

pub fn parse(s: &str) -> Option<Position> {
    let parts: Vec<&str> = s.trim().split('/').collect();
    if parts.len() != 6 {
        return None;
    }
    let turn = match parts[0] {
        "S" => Owner::Sente,
        "G" => Owner::Gote,
        _ => return None,
    };
    let mut board = [None; 12];
    for r in 0..4 {
        let row: Vec<char> = parts[1 + r].chars().collect();
        if row.len() != 3 {
            return None;
        }
        for f in 0..3 {
            if row[f] != '-' {
                board[r * 3 + f] = Some(char_piece(row[f])?);
            }
        }
    }
    let mut hand_sente = [0u8; 3];
    let mut hand_gote = [0u8; 3];
    if parts[5] != "-" {
        for c in parts[5].chars() {
            let (pc, o) = char_piece(c)?;
            match o {
                Owner::Sente => hand_sente[pc.hand_index()] += 1,
                Owner::Gote => hand_gote[pc.hand_index()] += 1,
            }
        }
    }
    Some(Position {
        board,
        hand_sente,
        hand_gote,
        turn,
    })
}

pub fn format(p: &Position) -> String {
    let mut s = String::new();
    s.push(match p.turn {
        Owner::Sente => 'S',
        Owner::Gote => 'G',
    });
    for r in 0..4 {
        s.push('/');
        for f in 0..3 {
            match p.board[r * 3 + f] {
                None => s.push('-'),
                Some((pc, o)) => {
                    let ch = piece_char(pc);
                    s.push(if o == Owner::Sente {
                        ch
                    } else {
                        ch.to_ascii_lowercase()
                    });
                }
            }
        }
    }
    s.push('/');
    let mut hand = String::new();
    for (i, &pc) in HAND_PIECES.iter().enumerate() {
        for _ in 0..p.hand_sente[i] {
            hand.push(piece_char(pc));
        }
    }
    for (i, &pc) in HAND_PIECES.iter().enumerate() {
        for _ in 0..p.hand_gote[i] {
            hand.push(piece_char(pc).to_ascii_lowercase());
        }
    }
    if hand.is_empty() {
        hand.push('-');
    }
    s.push_str(&hand);
    s
}

fn sq_name(sq: u8) -> String {
    let f = (b'a' + file(sq) as u8) as char;
    let r = (b'1' + rank(sq) as u8) as char;
    format!("{}{}", f, r)
}

/// clausecker-style move notation (e.g. "Cb3xb2", "Gc4-c3", "C*b1", trailing "+").
pub fn notation(mv: &Move) -> String {
    let mut s = String::new();
    s.push(piece_char(mv.piece));
    match mv.from {
        None => {
            s.push('*');
            s.push_str(&sq_name(mv.to));
        }
        Some(from) => {
            s.push_str(&sq_name(from));
            s.push(if mv.capture { 'x' } else { '-' });
            s.push_str(&sq_name(mv.to));
        }
    }
    if mv.promote {
        s.push('+');
    }
    s
}

/// Pack a position losslessly into a u64: 12 board cells (4 bits each) + 6 hand
/// counts (2 bits each) + the turn bit (61 bits used). A bijection on valid
/// positions, used as a compact hash key during the solve.
pub fn pack(p: &Position) -> u64 {
    let mut x = 0u64;
    for sq in 0..12 {
        let code: u64 = match p.board[sq] {
            None => 0,
            Some((pc, o)) => {
                let base = match pc {
                    Piece::Lion => 1,
                    Piece::Giraffe => 3,
                    Piece::Elephant => 5,
                    Piece::Chick => 7,
                    Piece::Hen => 9,
                };
                base + if o == Owner::Gote { 1 } else { 0 }
            }
        };
        x |= code << (sq * 4);
    }
    let mut bit = 48;
    for &c in p.hand_sente.iter().chain(p.hand_gote.iter()) {
        x |= (c as u64) << bit;
        bit += 2;
    }
    if p.turn == Owner::Gote {
        x |= 1 << 60;
    }
    x
}

pub fn unpack(x: u64) -> Position {
    let mut board = [None; 12];
    for sq in 0..12 {
        let code = (x >> (sq * 4)) & 0xF;
        if code != 0 {
            let owner = if code % 2 == 0 {
                Owner::Gote
            } else {
                Owner::Sente
            };
            let pc = match (code + 1) / 2 {
                1 => Piece::Lion,
                2 => Piece::Giraffe,
                3 => Piece::Elephant,
                4 => Piece::Chick,
                _ => Piece::Hen,
            };
            board[sq] = Some((pc, owner));
        }
    }
    let mut h = [0u8; 6];
    for i in 0..6 {
        h[i] = ((x >> (48 + 2 * i)) & 0x3) as u8;
    }
    let turn = if (x >> 60) & 1 == 1 {
        Owner::Gote
    } else {
        Owner::Sente
    };
    Position {
        board,
        hand_sente: [h[0], h[1], h[2]],
        hand_gote: [h[3], h[4], h[5]],
        turn,
    }
}

/// Turn-normalized equivalent (always Sente to move). A Gote-to-move position is
/// rotated 180° with its colors swapped, which is value-equivalent; two
/// turn-symmetric positions share this canonical form, halving the state space.
pub fn canonical(p: &Position) -> Position {
    if p.turn == Owner::Sente {
        return *p;
    }
    let mut board = [None; 12];
    for s in 0..12usize {
        let s2 = (3 - s / 3) * 3 + (2 - s % 3);
        board[s2] = p.board[s].map(|(pc, o)| (pc, o.flip()));
    }
    Position {
        board,
        hand_sente: p.hand_gote,
        hand_gote: p.hand_sente,
        turn: Owner::Sente,
    }
}

/// File-mirror (a↔c): a value-preserving spatial symmetry of the board.
fn mirror(p: &Position) -> Position {
    let mut board = [None; 12];
    for s in 0..12usize {
        board[(s / 3) * 3 + (2 - s % 3)] = p.board[s];
    }
    Position {
        board,
        hand_sente: p.hand_sente,
        hand_gote: p.hand_gote,
        turn: p.turn,
    }
}

/// Canonical packed key, folding the full value-symmetry group: turn (180°
/// rotation + colour swap) and left-right mirror. Equivalent positions share it.
pub fn canonical_key(p: &Position) -> u64 {
    let q = canonical(p); // turn-normalized (Sente to move)
    pack(&q).min(pack(&mirror(&q)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn canonical_is_idempotent_and_sente_identity() {
        let p = parse("S/gle/-c-/-C-/ELG/-").unwrap();
        assert_eq!(canonical(&p), p); // already Sente to move
                                      // a Gote-to-move position canonicalizes to Sente and is stable
        let g = parse("G/gle/-c-/-C-/ELG/-").unwrap();
        let c = canonical(&g);
        assert_eq!(c.turn, Owner::Sente);
        assert_eq!(canonical(&c), c);
    }

    #[test]
    fn pack_roundtrips() {
        let p = parse("S/gle/-c-/-C-/ELG/-").unwrap();
        assert_eq!(unpack(pack(&p)), p);
        // a position with promotions and pieces in hand
        let q = parse("G/l-R/-e-/-C-/L--/Geg").unwrap();
        assert_eq!(unpack(pack(&q)), q);
        assert_ne!(pack(&p), pack(&q));
    }

    const INIT: &str = "S/gle/-c-/-C-/ELG/-";

    #[test]
    fn roundtrip() {
        assert_eq!(format(&parse(INIT).unwrap()), INIT);
    }

    #[test]
    fn opening_moves_match_oracle() {
        let p = parse(INIT).unwrap();
        let got: BTreeSet<String> = p.moves().iter().map(notation).collect();
        let want: BTreeSet<String> = ["Cb3xb2", "Gc4-c3", "Lb4-a3", "Lb4-c3"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(got, want);
    }

    #[test]
    fn chick_capture_child_matches_oracle() {
        let p = parse(INIT).unwrap();
        let mv = p
            .moves()
            .into_iter()
            .find(|m| notation(m) == "Cb3xb2")
            .unwrap();
        // clausecker's probe gives this child for Cb3xb2
        assert_eq!(format(&p.make(&mv)), "G/gle/-C-/---/ELG/C");
    }
}
