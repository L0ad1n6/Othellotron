use crate::game::{
    Board, 
    ROW_LOOKUP, 
    COLUMN_LOOKUP, 
    POSITIVE_DIAGONAL_LOOKUP, 
    NEGATIVE_DIAGONAL_LOOKUP
};

/// Function to play moves onto board
/// 
/// # Arguments
/// *   board: Pointer to board which moves will be played on
/// *   i: Index of move on main board
/// *   color: Color of inserted piece
/// 
/// Returns vector of pieces flipped, very last element is inserted piece
pub fn play(board: &mut Board, i: usize, color: bool) -> Vec<usize> {
    board.insert(i, color); // Adds piece onto board
    board.prev_color = color;

    let axis = [
        POSITIVE_DIAGONAL_LOOKUP[i],
        NEGATIVE_DIAGONAL_LOOKUP[i], 
        ROW_LOOKUP[i], 
        COLUMN_LOOKUP[i]
    ];

    let mut flips = vec![];
    for a in axis {
        let index = a.iter().position(|&pos| pos == i).unwrap(); // Should never be able to reach None case, want it to panic if it does
        let mut temp_flips = vec![];
        let mut sequences = [Vec::from(&a[..index]), Vec::from(&a[(index + 1)..])];
        sequences[0].reverse();

        for sequence in sequences {
            for &pos in &sequence {
                if board.color_pieces[&color].contains(&pos) { // End of flip sequence
                    break;
    
                } else if board.pieces[pos] == None || // Gap in sequence means end of sequence
                        pos == *sequence.last().unwrap() && // Sequence did not terminate
                        board.color_pieces[&!color].contains(&pos) { 
                    
                    temp_flips = vec![];
                    break;

                } else if board.color_pieces[&!color].contains(&pos) { // Valid tile to flip
                    temp_flips.push(pos)
                }
            }

            flips.extend(temp_flips.iter());
            temp_flips = vec![];
        }
    }

    for &flip in &flips { // Carryout flips
        board.flip(flip) 
    }

    flips.push(i);
    flips

}

// Untested *potentially* faster, 
// And *potentially* broken
#[allow(dead_code)]
pub fn play2(board: &mut Board, i: usize, color: bool) -> Vec<usize> {
    board.prev_color = color;
    board.insert(i, color);
    let axis = [COLUMN_LOOKUP[i], ROW_LOOKUP[i], NEGATIVE_DIAGONAL_LOOKUP[i], POSITIVE_DIAGONAL_LOOKUP[i]];
    let mut flips = vec![];

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
                                break; // terminate sequence, after flips found
                            } else {
                                flips.push(ax[pos])
                            },
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
                                break; // terminate sequence, after flips found
                            } else {
                                flips.push(ax[pos])
                            },
                            None => break // Piece in sequence can't be empty
                        }
                    }
                }
            }
            
        }
    }

    for &flip in &flips { // Carryout flips
        board.flip(flip) 
    }

    flips.push(i);
    flips
}

/// Function to undo play function. MUST USE SAME BOARD. There are 
/// no checks in this function, the function is an internal function only 
/// used during search. If function is used without the same conditions a
/// s the play functions, it WILL mess the board up and cause bugs 
/// that are hard to catch and hard to reproduce
/// 
/// # Arguments
/// *   board: Pointer to board where moves flips will be undone
/// *   color: Color of piece that was played
/// *   flips: Vector of piece indices that were flipped
pub fn unplay(board: &mut Board, color: bool, mut flips: Vec<usize>) {
    board.remove(flips.pop().unwrap(), color); // Remove inserted piece
    board.prev_color = !color; 
    // Might cause bug if two moves are played by one side in case of no moves being found
    // https://en.wikipedia.org/wiki/Ostrich_algorithm

    for i in flips {
        board.flip(i) // Re-flip flipped tiles
    }
}

