pub mod moves;
mod search;
mod evaluation;

pub use search::search;
pub use evaluation::{WEIGHT_LOOKUP, evaluate};

use crate::game::Board;
use crate::human::to_notation;
use moves::generate_moves;
use crate::game;

/// Bot play function. Combines search and playing out move on board for bot.
/// 
/// # Arguments
/// *   board: State of board
/// *   color: Color of bot
/// *   max_depth: Depth that is searched (Will switch to Iterative Deepening in the future)
pub fn play(board: &mut Board, color: bool, max_depth: u32 ) {
    if generate_moves(board, color).is_empty() {
        if generate_moves(board, !color).is_empty() {
            match board.piece_count[&color].cmp(&board.piece_count[&!color]) {
                std::cmp::Ordering::Greater => println!("You Won! Piece Ratio = {}:{} (You:Bot)", board.piece_count[&color], board.piece_count[&!color]),
                std::cmp::Ordering::Equal => println!("A It's a tie! Piece Ratio = {}:{} (You:Bot)", board.piece_count[&color], board.piece_count[&!color]),
                std::cmp::Ordering::Less => println!("You Lost! Piece Ratio = {}:{} (You:Bot)", board.piece_count[&color], board.piece_count[&!color]),
            }
            std::process::exit(0);
        } else {
            return
        }
    }

    let mut moves = search::search(board, max_depth, color);
    moves.sort_by_key(|(score, _)| *score);
    let m = moves.last().unwrap().1;

    // let mut m = match search::iterative_search(board, color, 2000) {
    //     Some(m) => m,
    //     None => panic!("Time was not sufficient to find moves increase thinking time")
    // };

    game::play(board, m, color);
    println!("{}{}", to_notation(m).0, to_notation(m).1);
    // println!("{:?}", flips[1..].iter().map(|&i| to_notation(i)));
}