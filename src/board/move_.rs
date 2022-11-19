use crate::board::{piece::Piece, utils::*};

pub fn find_direction_offset(piece: &Piece) -> std::ops::Range<usize> {
    let mut start = 0;
    let mut end = 4;

    match piece {
        Piece::Blue(false) => start = 2,
        Piece::Red(false) => end = 2,
        _ => {}
    }

    start..end
}

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct Move {
    pub start: usize,
    pub kills: Vec<(usize, Piece)>,
    pub should_king: bool,
    pub through: Vec<usize>,
    pub end: usize,
}

impl Move {}
