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
        (Piece::Lion, _) => &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)],
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
                out.push(Move { piece: pc, from: Some(sq), to: nsq, capture, promote });
            }
        }
        for (i, &pc) in HAND_PIECES.iter().enumerate() {
            if self.hand(me)[i] == 0 {
                continue;
            }
            for nsq in 0..12u8 {
                if self.board[nsq as usize].is_none() {
                    out.push(Move { piece: pc, from: None, to: nsq, capture: false, promote: false });
                }
            }
        }
        out
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
    let owner = if c.is_ascii_uppercase() { Owner::Sente } else { Owner::Gote };
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
    Some(Position { board, hand_sente, hand_gote, turn })
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
                    s.push(if o == Owner::Sente { ch } else { ch.to_ascii_lowercase() });
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    const INIT: &str = "S/gle/-c-/-C-/ELG/-";

    #[test]
    fn roundtrip() {
        assert_eq!(format(&parse(INIT).unwrap()), INIT);
    }

    #[test]
    fn opening_moves_match_oracle() {
        let p = parse(INIT).unwrap();
        let got: BTreeSet<String> = p.moves().iter().map(notation).collect();
        let want: BTreeSet<String> =
            ["Cb3xb2", "Gc4-c3", "Lb4-a3", "Lb4-c3"].iter().map(|s| s.to_string()).collect();
        assert_eq!(got, want);
    }

    #[test]
    fn chick_capture_child_matches_oracle() {
        let p = parse(INIT).unwrap();
        let mv = p.moves().into_iter().find(|m| notation(m) == "Cb3xb2").unwrap();
        // clausecker's probe gives this child for Cb3xb2
        assert_eq!(format(&p.make(&mv)), "G/gle/-C-/---/ELG/C");
    }
}
