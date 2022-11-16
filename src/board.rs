use crate::utils::CollectArray;

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

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}

impl Manager {
    pub fn new() -> Self {
        let mut manager = Manager {
            board: (0..64).map(|_| Piece::Empty).collect_array(),
            players: [Player::User, Player::User],
            made_moves: Vec::new(),
            turn: 0usize,

            blue: Vec::new(),
            red: Vec::new(),

            kill_move_present: false,
            winner: Piece::Empty,
            gameover: false,

            moves_without_kill: 0,
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

                self.board[index] = Piece::Red(false)
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

                self.board[index] = Piece::Blue(false)
            }
        }
    }
}
