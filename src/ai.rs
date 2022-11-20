use crate::board::{Manager, Move, Piece};
use rand::{thread_rng, Rng};

fn generate_all_moves(manager: &Manager) -> Vec<Move> {
    let mut generated_moves = Vec::new();

    for (_, index) in manager.get_pieces(manager.current_side()) {
        generated_moves.extend(manager.piece_moves(index).iter().map(|x| x.to_owned()));
    }

    generated_moves
}

pub fn find_best_move(manager: &Manager) -> Move {
    // finds the best possible move
    let mut my_manager: Manager = manager.to_owned();
    let possible_moves = generate_all_moves(&my_manager);

    let mut rng = thread_rng();
    possible_moves[rng.gen_range(0..possible_moves.len())].clone()
}
