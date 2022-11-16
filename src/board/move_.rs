use crate::board::{piece::Piece, utils::*};

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct Move {
    pub start: usize,
    pub kills: Vec<(usize, Piece)>,
    pub should_king: bool,
    pub through: Vec<usize>,
    pub end: usize,
}

impl Move {}
