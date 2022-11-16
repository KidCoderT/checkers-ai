use crate::board::Piece;

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct Move {
    pub start: u8,
    pub kills: Vec<(u8, Piece)>,
    pub should_king: bool,
    pub through: Vec<u8>,
    pub end: u8,
}

impl Move {}
