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

pub fn move_offset(offset_index: usize) -> i8 {
    let i = if offset_index / 2 == 1 { -1 } else { 1 };
    DIRECTIONAL_OFFSET[offset_index % 2] * i
}

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct Move {
    pub start: usize,
    pub kills: Vec<(usize, Piece)>,
    pub should_king: bool,
    pub through: Vec<usize>,
    pub end: usize,
}

impl Move {
    pub fn extend(&self, end: usize, new_kill: (usize, Piece), should_king: bool) -> Self {
        let mut kills = self.kills.clone();
        kills.push(new_kill);

        let mut through = self.through.clone();
        through.push(self.end);

        Move {
            start: self.start,
            kills,
            through,
            should_king,
            end,
        }
    }

    pub fn new_move(
        start: usize,
        end: usize,
        should_king: bool,
        kill: Option<(usize, Piece)>,
    ) -> Self {
        let kills = if kill == None {
            Vec::new()
        } else {
            vec![kill.unwrap()]
        };
        let through = Vec::new();

        Move {
            start,
            kills,
            through,
            should_king,
            end,
        }
    }
}
