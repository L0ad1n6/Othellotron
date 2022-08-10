use super::{
    Board, 
    play, 
    COLUMN_LOOKUP, 
    ROW_LOOKUP, 
    NEGATIVE_DIAGONAL_LOOKUP, 
    POSITIVE_DIAGONAL_LOOKUP
};

#[allow(dead_code)]
pub fn validate_unoptimized(board: &Board, i: usize, color: bool) -> bool {
    let mut board = board.clone();

    match board.pieces.get(i) {
        Some(c) => if c.is_some() {
            return false; // Tile already full
        },
        None => return false // Out of bounds
    }

    let piece_count = board.piece_count[&color];
    play(&mut board, i, color);
    board.piece_count[&color] > piece_count + 1
}

// Much faster version

/// Move validation function, runs until at least 1 valid flip is found,
/// essentially the play function with preliminary checks and stops at first
/// found valid flip. The board is not and should not be modified by this 
/// function EVER, for the evaluation to be accurate
/// 
/// # Arguments
/// *   board: Board which is used to validate a move
/// *   i: Index of move on main board
/// *   color: Color of piece to be inserted
pub fn validate(board: &Board, i: usize, color: bool) -> bool {
    if i > 63 || board.pieces[i].is_some() { return false } // Out of bounds / Location full

    let axis = [COLUMN_LOOKUP[i], ROW_LOOKUP[i], NEGATIVE_DIAGONAL_LOOKUP[i], POSITIVE_DIAGONAL_LOOKUP[i]];
    for ax in axis {
        let index = ax.iter().position(|&pos| pos == i).unwrap(); // Cannot be None
        let axis_length = ax.iter().len();

        match axis_length {
            1 => continue, // No flips possible on this axis
            2 => continue, // It is impossible for there to be a piece on the other side
            3 => if index == 1 { continue }, // If it is in the middle No flips are possible
            _ => {}
        }

        if index > 1 { // If it is second from edge there is no point in searching that direction 
            if let Some(c) = board.pieces[ax[index - 1]] {
                if c != color { // Adjacent piece must be opposite color
                    for pos in (0..=(index - 2)).rev() {
                        match board.pieces[ax[pos]] {
                            Some(c) => if c == color {
                                return true // terminated sequence possible, move possible
                            }, // Nothing happens if the piece is of the opposite color
                            None => break // Piece in sequence can't be empty
                        }
                    }
                }
            }
        }

        // index of last element is (axis_length - 1)
        if index < (axis_length - 2) { // If it is second from edge there is no point in searching that direction 
            if let Some(piece) = board.pieces[ax[index + 1]] {
                if piece != color { // Adjacent piece must be opposite color
                    for pos in (index + 2)..axis_length {
                        match board.pieces[ax[pos]] {
                            Some(c) => if c == color {
                                return true // terminated sequence possible, move possible
                            }, // Nothing happens if the piece is of the opposite color
                            None => break // Piece in sequence can't be empty
                        }
                    }
                }
            }
        }
    }

    false
}
