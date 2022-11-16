use crate::utils::CollectArray;
use itertools;

mod move_;
mod piece;
mod utils;

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

    pub blue: Vec<usize>,
    pub red: Vec<usize>,

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

                self.board[index] = Piece::Red(false);
                self.red.push(index);
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

                self.board[index] = Piece::Blue(false);
                self.blue.push(index);
            }
        }
    }

    pub fn play_move(&mut self, selected_move: Move) {
        let mut piece = self.board[selected_move.start];
        self.board[selected_move.start] = Piece::Empty;

        if piece.is_king() == Some(false)
            && (piece.is_red() && selected_move.end > 56
                || piece.is_blue() && selected_move.end < 8)
        {
            piece = piece.king();
        }

        self.board[selected_move.end] = piece;

        // for (index, _) in &selected_move.kills {
        //     self.delete_piece(*index);
        //     self.moves_without_kill = 0;
        // }

        // self.made_moves
        //     .push((selected_move, self.kill_move_present));
        // self.update_game_state()
        
        let mut items = itertools::chain(&mut self.red, &mut self.blue);
        if let Some(item) = items.find(|x| x == &&selected_move.start) {
            *item = selected_move.end;
        };
    }
}
