use std::io::Write;

use crate::game;
use crate::game::{Board, validate};
use crate::bot::moves::generate_moves;
use text_io::scan;

/// Human interface function. Combines user input, parsing and playing.
/// Unfortunately, the current state of this function only supports,
/// invalid moves written in the right way, when text_io::scan!() 
/// attempts to parse the wrong type it panics, the convenience of the 
/// library to me the programmer has made it the reason I have chosen to use it.
/// I apologize in advance to whoever is interested enough to read the code 
/// for this project and has likely chosen to ATTEMPT to beat the bot, if you 
/// have found yourself losing to the bot and the bot crashing due to a miss input.
/// If you enter an invalid move it will let you re-enter a valid move, 
/// thus if you are not happy with the row you typed in make the column 
/// invalid to re-enter another move
/// 
/// # Arguments
/// *   board: State of board
/// *   color: Color of bot
/// *   max_depth: Depth that is searched (Will switch to Iterative Deepening in the future)
#[allow(unused_must_use)] // To not get warnings from "std::io::stdout().flush()"
pub fn play(board: &mut Board, color: bool) {
    let row: usize; // Define type for row
    let column: char; // Define type for column

    if generate_moves(board, color).is_empty() {
        if generate_moves(board, !color).is_empty() { // Game Over
            match board.piece_count[&!color].cmp(&board.piece_count[&color]) {
                std::cmp::Ordering::Greater => println!("You Won! Piece Ratio = {}:{} (You:Bot)", board.piece_count[&!color], board.piece_count[&color]),
                std::cmp::Ordering::Equal => println!("B It's a tie! Piece Ratio = {}:{} (You:Bot)", board.piece_count[&!color], board.piece_count[&color]),
                std::cmp::Ordering::Less => println!("You Lost! Piece Ratio = {}:{} (You:Bot)", board.piece_count[&!color], board.piece_count[&color]),
            }
            std::process::exit(0); // Exit program
        } else {
            return
        }
    }

    // **** Taking user input to parse move into something that the crate::game::play function can use ****

    print!("Enter Row (Number): ");
    std::io::stdout().flush();
    scan!("{}", row); 
    // TODO: write standard library implementation for this to 
    // TODO-continuation: avoid panicking and losing progress in a game.

    if row > 8 {
        println!("Invalid Row, try again.");
        return play(board, color)
    }

    print!("Enter Column (Letter): ");
    std::io::stdout().flush();
    scan!("{}", column); // TODO: See previous scan!() call, roughly line 46


    if (column.to_ascii_lowercase() as u32) < 97 || (column.to_ascii_lowercase() as u32) > 104 {
        println!("Invalid Column, try again.");
        return play(board, color)
    }

    let i = to_index(row, column);
    
    if validate(board, i, color) { // Validates that move that was entered is actually valid
        game::play(board, i, color);
    } else {
        println!("Invalid move");
        play(board, color) // Next attempt at entering a valid move
    }

}

/// Converts human row and column to index on main board
pub fn to_index(row: usize, column: char) -> usize {
    (column.to_ascii_lowercase() as u32 - 97) as usize + (8 - row) * 8
}

/// Converts index on main board to human row and column
pub fn to_notation(i: usize) -> (usize, char) {
    (8 - (i / 8), std::char::from_u32(i as u32 % 8 + 97)
        .unwrap()
        .to_ascii_uppercase())
}
