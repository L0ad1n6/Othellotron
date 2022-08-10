use crate::game::{Board, validate, play, unplay};

/// Simple move generation function
/// 
/// # Arguments
/// *   board: Board state where moves can be played
/// *   color: Side which moves are valid for
/// 
/// Returns vector of moves
pub fn generate_moves(board: &Board, color: bool) -> Vec<usize> {
    // TODO: Only check positions adjacent to pieces in future
    // TODO: Make pruned positions Cached
    // pruned_positions(board)
    (0..64)
        .into_iter()
        .filter(|&i| validate(board, i, color))
        .collect()
}

/// Function to prune positions that are checked
/// Too slow to work
#[allow(dead_code)]
fn pruned_positions(board: &Board) -> Vec<usize>  {
    let offsets: [i32; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
    (0..64)
        .filter(|&i| {
            let mut neighbor_found = false;
            for offset in offsets {
                if i + offset > 0 && i + offset < 63 && board.pieces[(i + offset) as usize].is_some() {
                    neighbor_found = true;
                    break
                } 
            }
            neighbor_found
        })
        .map(|i| i as usize)
        .collect()
    
    
    // let mut tiles = vec![];
    // for i in 0..64 {
    //     let neighbors = [-9, -8, -7, -1, 1, 7, 8, 9];
    //     for offset in neighbors {
    //         if i + offset > 0 && i + offset < 63 && board.pieces[(i + offset) as usize].is_some() {
    //             tiles.push(i);
    //             break
    //         } 
    //     }
    // }


}

/// Simple test for seeing how many moves are evaluated at base of game tree
/// 
/// Ply 0: 4
/// Ply 1: 12
/// Ply 2: 56
/// Ply 3: 244
/// Ply 4: 1396
/// Ply 5: 8200
/// Ply 6: 55092
/// Ply 7: 390216
/// Ply 8: 3005264
/// Ply 9: 24571000
/// Ply 10: 212257448
/// Ply 11: 1939875880
/// 
/// # Arguments
/// *   board: Board state where moves can be played
/// *   Side which moves are valid for
/// 
/// Returns number of positions that would be evaluated
pub fn _ply_move_counter_test(board: &Board, depth: u32, color: bool) -> usize {
    let moves = generate_moves(board, color);

    if moves.is_empty() { return 0 }
    else if depth == 0 { return moves.len() }

    let mut board = board.clone();
    moves
        .into_iter()
        .map(|m| {
            let flips = play(&mut board, m, color);
            let count =  _ply_move_counter_test(&board, depth - 1, !color);
            unplay(&mut board, color, flips);
            count })
        .reduce(|acc, c| acc + c)
        .unwrap()
}