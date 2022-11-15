
#[derive(Copy, Clone, Hash, Eq, Debug, PartialEq)]
pub enum Piece {
    Blue(bool),
    Red(bool),
    Empty
}

#[derive(Copy, Clone, Eq, Debug, PartialEq)]
pub enum Player {
    Computer,
    User
}

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct Move {
    start: u8,
    kills: Vec<(u8, Piece)>,
    should_king: bool,
    through: Vec<u8>,
    end: u8,
}

pub struct Manager {
    pub board: [Piece; 64],
    pub players: [Player; 2], // blue, red
    made_moves: Vec<(Move, bool)>, // Move, Kill move present
    turn: usize,

    pub blue: Vec<u8>,
    pub red: Vec<u8>,

    kill_move_present: bool,
    pub gameover: bool,
    pub winner: Piece,

    moves_without_kill: u8,
}

impl Piece {}

impl Move {}

impl Manager {}