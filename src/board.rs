mod move_;
mod piece;

pub use move_::Move;
pub use piece::Piece;

#[derive(Copy, Clone, Eq, Debug, PartialEq)]
pub enum Player {
    Computer,
    User,
}

pub struct Manager {
    pub board: [Piece; 64],
    pub players: [Player; 2],      // blue, red
    made_moves: Vec<(Move, bool)>, // Move, Kill move present
    turn: usize,

    pub blue: Vec<u8>,
    pub red: Vec<u8>,

    kill_move_present: bool,
    pub gameover: bool,
    pub winner: Piece,

    moves_without_kill: u8,
}

impl Manager {}
