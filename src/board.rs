use crate::traits;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Piece {
    pub hollow: bool,
    pub square: bool,
    pub short: bool,
    pub black: bool,
}

impl From<u8> for Piece {
    fn from(v: u8) -> Self {
        Self {
            hollow: ((v >> 0) & 1) != 0,
            square: ((v >> 1) & 1) != 0,
            short: ((v >> 2) & 1) != 0,
            black: ((v >> 3) & 1) != 0,
        }
    }
}

impl From<Piece> for u8 {
    fn from(p: Piece) -> Self {
        ((p.hollow as u8) << 0)
            + ((p.square as u8) << 1)
            + ((p.short as u8) << 2)
            + ((p.black as u8) << 3)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Position {
    pub row: u8,
    pub col: u8,
}

impl From<u8> for Position {
    fn from(v: u8) -> Self {
        Self {
            row: v / 4,
            col: v % 4,
        }
    }
}

impl From<Position> for u8 {
    fn from(p: Position) -> Self {
        4 * p.row + p.col
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Action {
    pub position: Position,
    pub piece: Piece,
}

impl From<u8> for Action {
    fn from(v: u8) -> Self {
        Self {
            position: Position::from(v / 16),
            piece: Piece::from(v % 16),
        }
    }
}

impl From<Action> for u8 {
    fn from(a: Action) -> Self {
        16 * u8::from(a.position) + u8::from(a.piece)
    }
}

impl traits::Action for Action {}
