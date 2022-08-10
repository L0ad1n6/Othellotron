use crate::game::{Board, play, unplay};

use super::moves::{generate_moves, heuristic_order};
use super::evaluation::evaluate;

/// Bot play function. Combines search and playing out move on board for bot.
/// 
/// # Arguments
/// *   board: Board which is evaluated
/// *   depth: Depth left to search
/// *   color: Side whose moves are being evaluated
/// 
/// Returns vector of scores and moves
pub fn search(board: &Board, depth: u32, color: bool) -> Vec<(i32, usize)> {
    // TODO: Add support for iterative deepening
    // TODO: Add support for a transposition table, likely Rc<RefCell<HashMap<board, i32>>>
    let mut moves = generate_moves(board, color);
    heuristic_order(&mut moves);
    search_with_moves_inputted(board, depth, color, moves)
}

/// search function but moves are inputted
fn search_with_moves_inputted(board: &Board, depth: u32, color: bool, moves: Vec<usize>) -> Vec<(i32, usize)> {
    let mut board = board.clone();

    let beta = std::i32::MAX - 1;
    let alpha = std::i32::MIN + 1;

    let mut evaluated_moves = vec![];

    if moves.is_empty() {
        return vec![]
    }

    for m in moves {
        let flips = play(&mut board, m, color);
        let evaluation = -search_inner(board.clone(), if depth == 0 { 0 } else { depth - 1 }, !color, -beta, -alpha);
        unplay(&mut board, color, flips);

        evaluated_moves.push((evaluation, m));
    }

    evaluated_moves
}

/// Bot play function. Combines search and playing out move on board for bot.
/// 
/// # Arguments
/// *   board: Board which is evaluated
/// *   depth: Depth left to search
/// *   color: Side whose moves are being evaluated
/// *   alpha: Best score for that side (worst for opponent)
/// *   beta: Worst score fot that side (best for opponent)
/// 
/// Returns evaluation of board state, propagated from when depth reaches 0
fn search_inner(mut board: Board, depth: u32, color: bool, mut alpha: i32, beta: i32) -> i32 {
    let mut moves = generate_moves(&board, color);
    heuristic_order(&mut moves);
    
    // If the final depth is reached or no moves are possible the evaluation of the board is propagated back
    if depth == 0 || moves.is_empty() { return evaluate(&board, color) }

    for m in moves {
        let flips = play(&mut board, m, color); // Plays Move
        let evaluation = -search_inner(board.clone(), depth - 1, !color, -beta, -alpha); // Evaluates recursively
        unplay(&mut board, color, flips); // Un-plays Move

        if evaluation >= beta {
            // Move was too good, needs to be pruned
            return beta
        }

        // Checks if move is better than current best score
        alpha = alpha.max(evaluation)
    }

    // Returns best possible score of that route
    alpha
}
