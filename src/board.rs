use crate::utils::CollectArray;

pub enum GameState {
    PlayingGame,
    GameOver,
}

#[derive(Copy, Clone, Hash, Eq, Debug, PartialEq)]
pub enum Piece {
    Blue,
    Red,
    BlueKing,
    RedKing,
    Empty,
}

pub struct Move {
    start: u8,
    kills: Vec<u8>,
    move_through: Vec<u8>,
    make_king: bool,
    end: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub index: u8,
    pub contains: Piece,
}

impl Position {
    pub fn default(index: u8) -> Self {
        Position {
            index: index,
            contains: Piece::Empty,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.contains == Piece::Empty
    }
}

pub struct Manager {
    pub board: [Position; 64],
    made_moves: Vec<Move>,
    pub game_state: GameState,
    moves_without_kill: u32,
    pub side: Piece,
    pub winner: Piece,
}

impl Manager {
    pub fn new() -> Self {
        let mut board: [Position; 64] = (0..64).map(|i| Position::default(i)).collect_array();

        let mut manager = Manager {
            board: board,
            game_state: GameState::PlayingGame,
            made_moves: Vec::new(),
            moves_without_kill: 0,
            side: Piece::Blue,
            winner: Piece::Empty,
        };

        manager.setup_pieces();

        manager
    }

    pub fn setup_pieces(&mut self) {
        for i in 0..3 {
            let is_even = (i % 2) == 0;

            for j in 0..4 {
                let index = {
                    let mut ans = (i * 8) + (j * 2);
                    if is_even {
                        ans += 1
                    }

                    ans
                };

                self.board[index].contains = Piece::Red
            }
        }

        for i in 5..8 {
            let is_even = (i % 2) == 0;

            for j in 0..4 {
                let index = {
                    let mut ans = (i * 8) + (j * 2);
                    if is_even {
                        ans += 1
                    }

                    ans
                };

                self.board[index].contains = Piece::Blue
            }
        }
    }

    pub fn move_piece(&mut self, old_index: u8, new_index: u8) {
        if self.board[old_index as usize].contains == Piece::Empty {
            panic!("the piece to move cannot be empty")
        }
        if self.board[new_index as usize].contains != Piece::Empty {
            panic!("u cant move the piece to a filled location")
        }

        let old_piece = self.board[old_index as usize].contains;
        self.board[old_index as usize].contains = Piece::Empty;
        self.board[new_index as usize].contains = old_piece;
    }

    pub fn delete_piece(&mut self, index: u8) {
        if self.board[index as usize].contains == Piece::Empty {
            panic!("u cant delete an empty position")
        }
        self.board[index as usize].contains = Piece::Empty;
    }

    pub fn king_piece(&mut self, index: u8) {
        let piece = self.board[index as usize].contains;

        if piece == Piece::Empty {
            panic!("u cant king an empty piece")
        }
        // if index > 8 && index < 56 {panic!("u cant king a piece thats not at the edge of the board")}

        self.board[index as usize].contains = match piece {
            Piece::Blue => Piece::BlueKing,
            Piece::Red => Piece::RedKing,
            _ => {
                panic!("this should not be possible")
            }
        };
    }
}
