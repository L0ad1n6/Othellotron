use crate::bot::evaluation::WEIGHT_LOOKUP;

/// Very very simple, lightweight move sorting function.
/// Only factor currently considered is position on board,
/// in future this function will be made to resemble to state
/// evaluation function but further optimizations will be 
/// necessary before then
/// 
/// # Arguments
/// *   moves: Moves to be sorted
pub fn heuristic_order(moves: &mut [usize]) {
    moves.sort_by_key(|&i| WEIGHT_LOOKUP[i])
}