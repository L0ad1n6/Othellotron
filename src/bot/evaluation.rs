use std::cmp::Ordering;

use crate::game::Board;
use super::moves::generate_moves;

// Heatmap of where pieces should roughly be on the board.
pub const WEIGHT_LOOKUP: [i32; 64] = [ // Very rough should be improved in future iterations
    6, 3, 4, 4, 4, 4, 3, 6,
    3, 1, 1, 1, 1, 1, 1, 3,
    4, 1, 2, 2, 2, 2, 1, 4,
    4, 1, 2, 1, 1, 2, 1, 4,
    4, 1, 2, 1, 1, 2, 1, 4,
    4, 1, 2, 2, 2, 2, 1, 4,
    3, 1, 1, 1, 1, 1, 1, 3,
    6, 3, 4, 4, 4, 4, 3, 6
]; 

/// Function to evaluate board states.
/// # Arguments
/// *   board: Board which is evaluated
/// *   color: Perspective of evaluation
/// Returns evaluation of board state.
pub fn evaluate(board: &Board, color: bool) -> i32 {
    // TODO: Implement a way to detect volatility of board 
    // TODO: Implement a way to score pieces connect to side and close to sides
    // TODO: Optimize constants and factors
    
    let mut score = 0; // Score starts at 0
    let moves = generate_moves(board, color); // Gives idea of how many moves are possible
    let opponent_moves = generate_moves(board, !color); // Gives idea of how many moves are possible

    // If no moves are possible a constant value is subtracted, 
    // depending on the ratio of your moves to opponents moves
    // A variable amount is added.

    if !moves.is_empty() && !opponent_moves.is_empty() {
        score += ((moves.len() as f32 / opponent_moves.len() as f32) * 20f32) as i32;

    } else if moves.is_empty() && opponent_moves.is_empty() {
        match board.piece_count[&color].cmp(&board.piece_count[&!color]) {
            Ordering::Greater => return std::i32::MAX - 1, // We win
            _ => return std::i32::MIN + 1 // We lose or tie, (we don't want to tie, only winners here)
        }

    } else if !moves.is_empty() { // We have no moves
        score -= 50;

    } else { // Opponent has no moves
        score += 40;
    }

    // The more pieces there are on the board, the more it matters 
    // who has more pieces. Early game it is good to have few pieces 
    // to minimize opponents moves and maximize future flips
    let total_piece_ratio = board.piece_count[&color] as f32 + board.piece_count[&!color] as f32 / 64f32;
    let piece_ratio = board.piece_count[&color] as f32 / board.piece_count[&!color] as f32;
    
    score += ((piece_ratio * 20f32) * total_piece_ratio) as i32; // Adding pieces to the score
    

    board.pieces
        .into_iter()
        .enumerate()
        .map(|(i, piece)| {
            if let Some(c) = piece { 
                if c == color {  WEIGHT_LOOKUP[i] } 
                else { 7-WEIGHT_LOOKUP[i] } 
            } else { 0 } })
        .fold(score, |acc, weight| acc + weight)
}

