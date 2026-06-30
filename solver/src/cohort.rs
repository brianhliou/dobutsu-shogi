//! Clausecker's layered tablebase index, ported to our `Position`.
//!
//! A legal, non-terminal position (normalized to Sente-to-move and folded by
//! the horizontal mirror) maps bijectively onto `[0, POSITION_COUNT)` =
//! `[0, 167_527_962)`. The four levels are ownership × cohort × lionpos × map;
//! see clausecker's `dobutsutable.h`/`poscode.c` for the decomposition. We reuse
//! his proven tables (`cohort_tables.rs`, machine-generated) to match his exact
//! footprint, so the solve can store one byte per position instead of a hash map.
//!
//! Terminal positions (a lion capturable now, or a lion already ascended) are
//! *not* in this space; the solve detects those with the engine and never
//! encodes them.

use crate::{Owner, Piece, Position};

include!("cohort_tables.rs");

const IN_HAND: u8 = 12;
const GOTE_PIECE: u8 = 16;
const GOTE_MOVES: u32 = 1 << 8;

// piece slots, matching clausecker's enum order
const CHCK_S: usize = 0;
const GIRA_S: usize = 2;
const ELPH_S: usize = 4;
const LION_S: usize = 6;
const LION_G: usize = 7;

/// per-ownership block size = POSITION_TOTAL_COUNT / OWNERSHIP_TOTAL_COUNT
const BLOCK: u64 = POSITION_TOTAL_COUNT / OWNERSHIP_TOTAL_COUNT as u64;

/// His square-and-owner board representation: `pieces[i]` is a square (0..12,
/// 12 = in hand) plus `GOTE_PIECE` if Gote owns it. `status` holds the rooster
/// (promotion) bits and the Gote-to-move bit.
#[derive(Clone, Copy)]
struct CPos {
    pieces: [u8; 8],
    status: u32,
}

#[inline]
fn gote_owns(v: u8) -> bool {
    v & GOTE_PIECE != 0
}

/// Our squares number from the top-left (rank 0 = top); clausecker numbers from
/// the bottom-right. The two differ by a 180° relabel: `his = 11 - ours`. In
/// hand (12) is unchanged. This is its own inverse.
#[inline]
fn remap(s: u8) -> u8 {
    if s == IN_HAND {
        IN_HAND
    } else {
        11 - s
    }
}

/// our hand array is [giraffe, elephant, chick]; clausecker's kinds are
/// chick(0), giraffe(1), elephant(2). this maps a kind index to our hand slot.
#[inline]
fn hand_slot(kind: usize) -> usize {
    match kind {
        0 => 2, // chick
        1 => 0, // giraffe
        2 => 1, // elephant
        _ => unreachable!(),
    }
}

/// Convert our `Position` into clausecker's piece-centric representation. The two
/// pieces of a kind go into adjacent slots in arbitrary order; his normalization
/// makes the order irrelevant to the resulting poscode.
fn to_cpos(p: &Position) -> CPos {
    let mut pieces = [IN_HAND; 8];
    let mut status = 0u32;
    if p.turn == Owner::Gote {
        status |= GOTE_MOVES;
    }

    // kinds: 0 chick (slots 0,1), 1 giraffe (slots 2,3), 2 elephant (slots 4,5)
    for kind in 0..3 {
        let base = kind * 2;
        let mut n = 0;
        // pieces on the board
        for (sq, cell) in p.board.iter().enumerate() {
            if let Some((piece, owner)) = cell {
                let (is_kind, promoted) = match (kind, piece) {
                    (0, Piece::Chick) => (true, false),
                    (0, Piece::Hen) => (true, true),
                    (1, Piece::Giraffe) => (true, false),
                    (2, Piece::Elephant) => (true, false),
                    _ => (false, false),
                };
                if is_kind {
                    let slot = base + n;
                    pieces[slot] = remap(sq as u8) | if *owner == Owner::Gote { GOTE_PIECE } else { 0 };
                    if promoted {
                        status |= 1 << slot;
                    }
                    n += 1;
                }
            }
        }
        // pieces in hand
        for owner in [Owner::Sente, Owner::Gote] {
            let count = p.hand(owner)[hand_slot(kind)];
            for _ in 0..count {
                let slot = base + n;
                pieces[slot] = IN_HAND | if owner == Owner::Gote { GOTE_PIECE } else { 0 };
                n += 1;
            }
        }
        debug_assert!(n <= 2, "more than two pieces of one kind");
    }

    // lions: exactly one of each, always on the board in a stored position
    for (sq, cell) in p.board.iter().enumerate() {
        if let Some((Piece::Lion, owner)) = cell {
            if *owner == Owner::Sente {
                pieces[LION_S] = remap(sq as u8);
            } else {
                pieces[LION_G] = remap(sq as u8) | GOTE_PIECE;
            }
        }
    }

    CPos { pieces, status }
}

#[inline]
fn gote_moves(c: &CPos) -> bool {
    c.status & GOTE_MOVES != 0
}

/// Turn the board 180°, exchanging Sente and Gote. Mirrors clausecker's
/// `turn_board`: square s -> GOTE_PIECE|(11-s), and the lions swap slots.
fn turn_board(c: &mut CPos) {
    #[inline]
    fn t(v: u8) -> u8 {
        let sq = v & !GOTE_PIECE;
        let gote = v & GOTE_PIECE;
        let nsq = if sq == IN_HAND { IN_HAND } else { 11 - sq };
        // flip owner
        nsq | (GOTE_PIECE - gote) & GOTE_PIECE
    }
    for i in 0..8 {
        c.pieces[i] = t(c.pieces[i]);
    }
    c.pieces.swap(LION_S, LION_G);
    c.status ^= GOTE_MOVES;
}

/// Vertically mirror along the centre file. Mirrors clausecker's `mirror_board`:
/// file 0<->2 within each rank, owner and in-hand unchanged.
fn mirror_board(c: &mut CPos) {
    #[inline]
    fn m(v: u8) -> u8 {
        let sq = v & !GOTE_PIECE;
        let gote = v & GOTE_PIECE;
        if sq == IN_HAND {
            return v;
        }
        let rank = sq / 3;
        let file = sq % 3;
        (rank * 3 + (2 - file)) | gote
    }
    for i in 0..8 {
        c.pieces[i] = m(c.pieces[i]);
    }
}

/// Normalize: turn to Sente-to-move, then mirror under clausecker's condition.
fn normalize(c: &mut CPos) {
    if gote_moves(c) {
        turn_board(c);
    }
    // mirror if the Sente lion is on the C file, or it is on the B file and the
    // Gote lion is on the A file. masks operate on 1<<piece-value.
    let ls = 1u32 << c.pieces[LION_S];
    let lg = 1u32 << c.pieces[LION_G];
    const SENTE_C: u32 = 0o444; // squares 2,5,8
    const SENTE_B: u32 = 0o2222; // squares 1,4,7,10
    const GOTE_A: u32 = 0o1111 << 16; // squares 0,3,6,9 owned by gote
    if ls & SENTE_C != 0 || (ls & SENTE_B != 0 && lg & GOTE_A != 0) {
        mirror_board(c);
    }
}

fn encode_ownership(c: &CPos) -> u32 {
    let mut r = 0u32;
    for (bit, &slot) in [CHCK_S, CHCK_S + 1, GIRA_S, GIRA_S + 1, ELPH_S, ELPH_S + 1]
        .iter()
        .enumerate()
    {
        if gote_owns(c.pieces[slot]) {
            r |= 1 << bit;
        }
    }
    r
}

#[inline]
fn remove_square(board_map: &mut [u8; 12], inverse_map: &mut [u8; 12], n: usize, pc: u8) {
    let sq = inverse_map[pc as usize];
    inverse_map[board_map[n] as usize] = sq;
    board_map[sq as usize] = board_map[n];
}

/// The result of encoding the piece placement: the four poscode fields.
struct PosCode {
    ownership: u32,
    cohort: u32,
    lionpos: u32,
    map: u32,
}

/// Port of clausecker's `encode_pieces`. Consumes a *normalized* position.
fn encode_pieces(c: &mut CPos, mut ownership: u32) -> PosCode {
    let mut code: u32 = 0;
    let mut squares: usize = 12;
    let mut oswap: u32 = 0;
    let mut cohortbits: u32 = 0;

    let mut board_map: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut inverse_map: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    // erase ownership, leaving square numbers
    for i in 0..8 {
        c.pieces[i] &= !GOTE_PIECE;
    }

    let lionpos = LIONPOS_MAP[c.pieces[LION_S] as usize][(c.pieces[LION_G] - 3) as usize];
    debug_assert!(lionpos >= 0, "invalid lion configuration");
    let lionpos = lionpos as u32;

    if c.pieces[LION_S] > c.pieces[LION_G] {
        squares -= 1;
        remove_square(&mut board_map, &mut inverse_map, squares, c.pieces[LION_S]);
        squares -= 1;
        remove_square(&mut board_map, &mut inverse_map, squares, c.pieces[LION_G]);
    } else {
        squares -= 1;
        remove_square(&mut board_map, &mut inverse_map, squares, c.pieces[LION_G]);
        squares -= 1;
        remove_square(&mut board_map, &mut inverse_map, squares, c.pieces[LION_S]);
    }

    let mut i = 0;
    while i < 6 {
        if c.pieces[i + 1] == IN_HAND {
            if c.pieces[i] == IN_HAND {
                // both in hand: normalize ownership only
                if ownership & (3 << i) == 2u32 << i {
                    oswap |= 3 << i;
                }
            } else {
                // one piece, no swap
                cohortbits |= 1 << i;
                code = code * squares as u32 + inverse_map[c.pieces[i] as usize] as u32;
                squares -= 1;
                remove_square(&mut board_map, &mut inverse_map, squares, c.pieces[i]);
            }
        } else if c.pieces[i] == IN_HAND {
            // one piece, swap
            oswap |= 3 << i;
            cohortbits |= 1 << i;
            code = code * squares as u32 + inverse_map[c.pieces[i + 1] as usize] as u32;
            squares -= 1;
            remove_square(&mut board_map, &mut inverse_map, squares, c.pieces[i + 1]);
        } else {
            // two pieces
            cohortbits |= 3 << i;
            let mut high = inverse_map[c.pieces[i] as usize] as u32;
            let mut low = inverse_map[c.pieces[i + 1] as usize] as u32;
            if high < low {
                oswap |= 3 << i;
                std::mem::swap(&mut high, &mut low);
            }
            code = code * PAIR_MAP[squares - 1] as u32 + PAIR_MAP[high as usize - 1] as u32 + low;
            squares -= 1;
            remove_square(&mut board_map, &mut inverse_map, squares, high as u8);
            squares -= 1;
            remove_square(&mut board_map, &mut inverse_map, squares, low as u8);
        }
        i += 2;
    }

    // fix ownership and promotion bits
    ownership ^= oswap & OWNER_FLIP[ownership as usize] as u32;
    if oswap & 3 != 0 {
        c.status = (c.status & !3) | PROM_FLIP[(c.status & 3) as usize] as u32;
    }

    cohortbits |= (c.status & 3) << 6;
    let cohort = COHORT_MAP[cohortbits as usize];
    debug_assert!(cohort >= 0, "invalid cohort");

    PosCode {
        ownership,
        cohort: cohort as u32,
        lionpos,
        map: code,
    }
}

#[inline]
fn position_offset(pc: &PosCode) -> u64 {
    // Unfolded layout: every ownership class in [0, 64) gets its own block, so
    // every legal non-terminal position has a slot in [0, POSITION_TOTAL_COUNT).
    // (The Sente>=Gote ownership fold that would shrink this to POSITION_COUNT is
    // a separate refinement layered on top.)
    let cs = &COHORT_SIZE[pc.cohort as usize];
    pc.ownership as u64 * BLOCK
        + cs.offset as u64
        + cs.size as u64 * pc.lionpos as u64
        + pc.map as u64
}

/// Whether an (ownership, cohort) pair actually occurs. Slots failing this are
/// allocated but never correspond to a real position (the in-hand same-kind
/// ownership normalization rules them out); the solve skips them.
#[inline]
pub fn is_valid_ownership(ownership: u32, cohort: u32) -> bool {
    VALID_OWNERSHIP_MAP[cohort as usize] & (1u64 << ownership) != 0
}

/// Encode a legal, non-terminal position to its dense tablebase offset in
/// `[0, POSITION_COUNT)`.
pub fn encode(p: &Position) -> u64 {
    let mut c = to_cpos(p);
    normalize(&mut c);
    let ownership = encode_ownership(&c);
    let pc = encode_pieces(&mut c, ownership);
    debug_assert!((pc.lionpos as usize) < LIONPOS_COUNT as usize);
    position_offset(&pc)
}

/// Like `encode`, but returns `None` if the position is terminal in the index
/// sense (lions adjacent => lionpos >= LIONPOS_COUNT). Used so callers can skip
/// positions that aren't stored.
pub fn encode_checked(p: &Position) -> Option<u64> {
    let mut c = to_cpos(p);
    normalize(&mut c);
    let ls = (c.pieces[LION_S] & !GOTE_PIECE) as usize;
    let lg = (c.pieces[LION_G] & !GOTE_PIECE) as usize;
    if ls >= LIONPOS_MAP.len() || lg < 3 {
        return None;
    }
    let lionpos = LIONPOS_MAP[ls][lg - 3];
    if lionpos < 0 || lionpos as u32 >= LIONPOS_COUNT {
        return None;
    }
    let ownership = encode_ownership(&c);
    let pc = encode_pieces(&mut c, ownership);
    if pc.lionpos >= LIONPOS_COUNT {
        return None;
    }
    Some(position_offset(&pc))
}

// ---- decode ----

fn assign_ownership(pieces: &mut [u8; 8], os: u32) {
    for (bit, &slot) in [CHCK_S, CHCK_S + 1, GIRA_S, GIRA_S + 1, ELPH_S, ELPH_S + 1]
        .iter()
        .enumerate()
    {
        if os & (1 << bit) != 0 {
            pieces[slot] |= GOTE_PIECE;
        }
    }
    pieces[LION_G] |= GOTE_PIECE;
}

/// Port of clausecker's `place_pieces`: fill pieces[] from cohort/lionpos/map.
/// Same-kind order is indeterminate (fixed up when converting to our Position).
fn place_pieces(cohort: usize, lionpos: usize, mut map: u32) -> CPos {
    let ci = &COHORT_INFO[cohort];
    let mut code = [0u32; 3];
    code[2] = map % ci.sizes[2] as u32;
    map /= ci.sizes[2] as u32;
    code[1] = map % ci.sizes[1] as u32;
    map /= ci.sizes[1] as u32;
    code[0] = map;

    let mut pieces = [IN_HAND; 8];
    let mut board_map: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut squares = 12usize;

    let hi = LIONPOS_INVERSE[lionpos][0];
    let lo = LIONPOS_INVERSE[lionpos][1];
    pieces[LION_S] = hi;
    pieces[LION_G] = lo;
    if hi > lo {
        squares -= 1;
        board_map[hi as usize] = board_map[squares];
        squares -= 1;
        board_map[lo as usize] = board_map[squares];
    } else {
        squares -= 1;
        board_map[lo as usize] = board_map[squares];
        squares -= 1;
        board_map[hi as usize] = board_map[squares];
    }

    for i in 0..3 {
        match ci.pieces[i] {
            0 => {
                pieces[2 * i] = IN_HAND;
                pieces[2 * i + 1] = IN_HAND;
            }
            1 => {
                pieces[2 * i] = board_map[code[i] as usize];
                pieces[2 * i + 1] = IN_HAND;
                squares -= 1;
                board_map[code[i] as usize] = board_map[squares];
            }
            2 => {
                let high = PAIR_INVERSE[code[i] as usize] as u32;
                let low = code[i] - PAIR_MAP[high as usize] as u32;
                pieces[2 * i] = board_map[(high + 1) as usize];
                pieces[2 * i + 1] = board_map[low as usize];
                squares -= 1;
                board_map[(high + 1) as usize] = board_map[squares];
                squares -= 1;
                board_map[low as usize] = board_map[squares];
            }
            _ => unreachable!(),
        }
    }

    CPos {
        pieces,
        status: ci.status as u32,
    }
}

/// Convert clausecker's representation back into our `Position` (Sente to move).
fn from_cpos(c: &CPos) -> Position {
    let mut board: [Option<(Piece, Owner)>; 12] = [None; 12];
    let mut hand_sente = [0u8; 3];
    let mut hand_gote = [0u8; 3];

    for slot in 0..8 {
        let v = c.pieces[slot];
        let sq = (v & !GOTE_PIECE) as usize;
        let owner = if gote_owns(v) { Owner::Gote } else { Owner::Sente };
        let kind = slot / 2; // 0 chick, 1 giraffe, 2 elephant, 3 lion
        if sq == IN_HAND as usize {
            let h = if owner == Owner::Sente {
                &mut hand_sente
            } else {
                &mut hand_gote
            };
            h[hand_slot(kind)] += 1;
        } else {
            let piece = match kind {
                0 => {
                    if c.status & (1 << slot) != 0 {
                        Piece::Hen
                    } else {
                        Piece::Chick
                    }
                }
                1 => Piece::Giraffe,
                2 => Piece::Elephant,
                3 => Piece::Lion,
                _ => unreachable!(),
            };
            board[remap(sq as u8) as usize] = Some((piece, owner));
        }
    }

    Position {
        board,
        hand_sente,
        hand_gote,
        turn: Owner::Sente,
    }
}

/// Split a dense offset into its four poscode fields.
fn split(offset: u64) -> (u32, usize, usize, u32) {
    let ownership = (offset / BLOCK) as u32;
    let within = offset % BLOCK;

    // largest cohort whose offset <= within
    let mut cohort = 0usize;
    for c in 0..COHORT_COUNT {
        if COHORT_SIZE[c].offset as u64 <= within {
            cohort = c;
        } else {
            break;
        }
    }
    let cs = &COHORT_SIZE[cohort];
    let rem = within - cs.offset as u64;
    let lionpos = (rem / cs.size as u64) as usize;
    let map = (rem % cs.size as u64) as u32;
    (ownership, cohort, lionpos, map)
}

/// Decode a dense offset back into our `Position`, or `None` if the slot is an
/// invalid-ownership allocation that no real position maps to.
pub fn decode_checked(offset: u64) -> Option<Position> {
    let (ownership, cohort, lionpos, map) = split(offset);
    if !is_valid_ownership(ownership, cohort as u32) {
        return None;
    }
    let mut c = place_pieces(cohort, lionpos, map);
    assign_ownership(&mut c.pieces, ownership);
    Some(from_cpos(&c))
}

/// Decode a dense offset known to hold a valid position.
pub fn decode(offset: u64) -> Position {
    decode_checked(offset).expect("offset is a valid position slot")
}

/// Cheap check (no piece placement) of whether an offset holds a real position.
pub fn is_valid_offset(offset: u64) -> bool {
    let (ownership, cohort, _, _) = split(offset);
    is_valid_ownership(ownership, cohort as u32)
}
