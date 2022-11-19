use crate::utils::CollectArray;
use itertools;

mod move_;
mod piece;
mod utils;

pub use move_::Move;
pub use piece::Piece;

use self::move_::find_direction_offset;

#[derive(Copy, Clone, Eq, Debug, PartialEq)]
pub enum Player {
    Computer,
    User,
}
pub struct Manager {
    pub board: [Piece; 64],
    pub players: [Player; 2],          // blue, red
    
    pub gameover: bool,
    pub winner: Piece,
    
    made_moves: Vec<(Move, bool, u8)>, // Move, Kill move present, moves without kills
    kill_move_present: bool,
    moves_without_kill: u8,
    turn: usize,
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

            kill_move_present: false,
            winner: Piece::Empty,
            gameover: false,

            moves_without_kill: 0,
        };

        manager.setup_pieces();

        manager
    }

    pub fn current_side(&self) -> Piece {
        match self.turn % 2 {
            0usize => Piece::Blue(false),
            1usize => Piece::Red(false),
            _ => {
                panic!("this cannot happen u messed up")
            }
        }
    }

    pub fn get_pieces(&self, side: Piece) -> Vec<(Piece, usize)> {
        let mut pieces: Vec<(Piece, usize)> = Vec::new();
        for (index, piece) in self.board.iter().enumerate() {
            if piece.is_empty() || !piece.match_piece(&side) {continue}
            pieces.push((piece.to_owned(), index));
        }

        pieces
    }

    fn setup_side(&mut self, start: usize, end: usize, piece: Piece) {
        for i in start..end {
            let is_even = (i % 2) == 0;

            for j in 0..4 {
                let index = {
                    let mut ans = (i * 8) + (j * 2);
                    if is_even {
                        ans += 1
                    }

                    ans
                };

                self.board[index] = piece;
            }
        }
    }

    pub fn setup_pieces(&mut self) {
        self.setup_side(0, 3, Piece::Red(false));
        self.setup_side(5, 8, Piece::Blue(false));
    }

    pub fn play_move(&mut self, selected_move: Move) {
        let mut piece = self.board[selected_move.start];
        self.board[selected_move.start] = Piece::Empty;

        if selected_move.should_king {
            piece = piece.king();
        }

        self.board[selected_move.end] = piece;

        for (index, _) in &selected_move.kills {
            self.board[*index] = Piece::Empty;
        }

        self.moves_without_kill = match selected_move.kills.is_empty() {
            false => self.moves_without_kill + 1, // increment by 1 if kill is present
            true => 0,                            // set the selected moves to 0
        };

        self.made_moves.push((
            selected_move,
            self.kill_move_present,
            self.moves_without_kill,
        ));

        self.update_state()
    }

    pub fn undo_move(&mut self) {
        let (last_move, kill_move_present, moves_without_kill) = self.made_moves.pop().unwrap();
        self.kill_move_present = kill_move_present;
        self.moves_without_kill = moves_without_kill;
        self.turn -= 1;

        let mut piece = self.board[last_move.end];
        self.board[last_move.end] = Piece::Empty;

        if last_move.should_king {
            piece = piece.base_form();
        }

        self.board[last_move.start] = piece;

        for (index, piece) in &last_move.kills {
            self.board[*index] = piece.to_owned();
        }
    }
    
    fn update_state(&mut self) {
        self.kill_move_present = false; // todo: implement this check
        self.turn += 1;

        for (piece, index) in self.get_pieces(self.current_side()) {
            for offset_index in find_direction_offset(&piece) {
                let move_offset: i8 = {
                    let i = if offset_index / 2 == 1 {-1} else {1};
                    utils::DIRECTIONAL_OFFSET[offset_index % 2] * i
                };

                if utils::NUM_SQUARES_TO_EDGE[index][offset_index] >= 2 {
                    let kill_index = (index as i8 + move_offset) as usize;
                    let move_to_index = (kill_index as i8 + move_offset) as usize;

                    if self.board[move_to_index].is_empty()
                        && self.board[kill_index].match_piece(&piece.opposite())
                    {
                        self.kill_move_present = true;
                        break;
                    }
                }
            }
            
            if self.kill_move_present {break}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Manager,
        Piece
    };

    #[test]
    fn test_get_pieces() {
        let manager = Manager::new();
        let pieces = manager.get_pieces(Piece::Red(false));

        assert_eq!(pieces.len(), 12);

        for (piece, _) in pieces.iter() {
            assert!(!piece.is_empty());
            assert!(piece.is_red())
        }
    }
}