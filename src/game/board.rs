use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use colored::Colorize;
use crate::bot::moves::generate_moves;

type Piece = Option<bool>;

/// Board data type, uses multiple channels to not have 
/// to redundantly iterate of main board. Future versions 
/// of this bot might see this data structure converted to 
/// 2 u64's as bitmaps. Despite bitmaps being much more memory 
/// efficient I believe this code is more readable and the 
/// current efficiency of the bot is sufficient for my project 
/// goals.
/// 
/// Takes no parameters while initializing.
#[derive(Eq, PartialEq)]
pub struct Board {
    pub pieces: [Piece; 64],
    pub color_pieces: HashMap<bool, HashSet<usize>>,
    pub piece_count: HashMap<bool, u8>,
    pub prev_color: bool
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: [
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, Some(true), Some(false), None, None, None,
                None, None, None, Some(false), Some(true), None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None
            ],
            color_pieces: HashMap::from([(true, HashSet::from([27, 36])), (false, HashSet::from([28, 35]))]),
            piece_count: HashMap::from([(true, 2), (false, 2)]),
            prev_color: false
        }
    }
}

// Implementation of helper functions for playing out moves on the board
impl Board {
    /// Inserts piece onto board
    pub fn insert(&mut self, i: usize, color: bool) {
        self.pieces[i] = Some(color); // Adding piece to main board

        // Adding piece to color specific storage
        self.color_pieces
            .get_mut(&color)
            .unwrap()
            .insert(i);

        // Updating piece count
        *self.piece_count
            .get_mut(&color)
            .unwrap() += 1;
    } 

    /// Removes piece from board
    pub fn remove(&mut self, i: usize, color: bool) {
        self.pieces[i] = None; // Removing piece from main board

        // Removing piece from color specific storage
        self.color_pieces
            .get_mut(&color)
            .unwrap()
            .remove(&i);

        // Updating piece count
        *self.piece_count
            .get_mut(&color)
            .unwrap() -= 1;
    } 

    /// Flips color of piece on board, 
    /// nothing happens if there isn't a piece
    pub fn flip(&mut self, i: usize) {
        // Determine initial color
        let color = match self.pieces[i] {
            Some(color) => color,
            None => return eprintln!("No pieces on tile to flip")
        };

        // Flips color on main board
        self.pieces[i] = Some(!color);

        // Update piece on original color storage
        self.color_pieces
            .get_mut(&color)
            .unwrap()
            .remove(&i);

        // Update piece count
        *self.piece_count
            .get_mut(&color)
            .unwrap() -= 1;

        // Update piece on target color storage
        self.color_pieces
            .get_mut(&!color)
            .unwrap()
            .insert(i);

        // Update piece count
        *self.piece_count
            .get_mut(&!color)
            .unwrap() += 1;
    }
}

// Implementing display for board to make it easy to print and visualize
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /*
          A   B   C   D   E   F   G   H
          +---+---+---+---+---+---+---+---+
          ...
        4 | {} | {} | {} | {} | {} | {} | {} | {} |
          ...
          +---+---+---+---+---+---+---+---+
        */
        
        let mut board = String::from("\n    A   B   C   D   E   F   G   H\n  +---+---+---+---+---+---+---+---+\n");
        let moves = generate_moves(self, !self.prev_color);
        
        for row in 0..8 {
            board.push_str(&format!("{} |", 8 - row));
            for column in 0..8 {
                let symbol= match self.pieces[row*8 + column] {
                    Some(color) => { 
                        if color { 
                            "W".white()
                        } else { 
                            "B".black()
                        } 
                    },
                    None => {
                        if moves.contains(&(row*8 + column)) {
                            "X".magenta()
                        } else {
                            " ".clear()
                        }
                    }
                };
                
                board.push_str(&format!(" {} |", symbol));
            }
            board.push_str("\n  +---+---+---+---+---+---+---+---+\n")
        }

        write!(f, "{board}")?;
        write!(f, "Score (White:Black): {}:{}", self.piece_count[&true], self.piece_count[&false])?;
        Ok(())
    }
}

// Implementing clone for board state
impl Clone for Board {
    fn clone(&self) -> Self {
        Self {
            pieces: self.pieces,
            color_pieces: self.color_pieces.clone(),
            piece_count: self.piece_count.clone(),
            prev_color: self.prev_color
        }
    }
}

// Implementing hash for board state (For transposition table hashmap)
impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pieces.hash(state);
        self.prev_color.hash(state)
    }
}


// **** All possible values for diagonals, rows and column ****
const P_DIAGONALS: [&[usize]; 15] = [
    &[0],
    &[8, 1],
    &[16, 9, 2],
    &[24, 17, 10, 3],
    &[32, 25, 18, 11, 4],
    &[40, 33, 26, 19, 12, 5],
    &[48, 41, 34, 27, 20, 13, 6],
    &[56, 49, 42, 35, 28, 21, 14, 7],
    &[57, 50, 43, 36, 29, 22, 15],
    &[58, 51, 44, 37, 30, 23],
    &[59, 52, 45, 38, 31],
    &[60, 53, 46, 39],
    &[61, 54, 47],
    &[62, 55],
    &[63]
];

const N_DIAGONALS: [&[usize]; 15] = [
    &[56],
    &[48, 57],
    &[40, 49, 58],
    &[32, 41, 50, 59],
    &[24, 33, 42, 51, 60],
    &[16, 25, 34, 43, 52, 61],
    &[8, 17, 26, 35, 44, 53, 62],
    &[0, 9, 18, 27, 36, 45, 54, 63],
    &[1, 10, 19, 28, 37, 46, 55],
    &[2, 11, 20, 29, 38, 47],
    &[3, 12, 21, 30, 39],
    &[4, 13, 22, 31],
    &[5, 14, 23],
    &[6, 15],
    &[7]

];

const ROWS: [&[usize]; 8] = [
    &[0, 1, 2, 3, 4, 5, 6, 7],
    &[8, 9, 10, 11, 12, 13, 14, 15],
    &[16, 17, 18, 19, 20, 21, 22, 23],
    &[24, 25, 26, 27, 28, 29, 30, 31],
    &[32, 33, 34, 35, 36, 37, 38, 39],
    &[40, 41, 42, 43, 44, 45, 46, 47],
    &[48, 49, 50, 51, 52, 53, 54, 55],
    &[56, 57, 58, 59, 60, 61, 62, 63]
];

const COLUMNS: [&[usize]; 8] = [
    &[0, 8, 16, 24, 32, 40, 48, 56],
    &[1, 9, 17, 25, 33, 41, 49, 57],
    &[2, 10, 18, 26, 34, 42, 50, 58],
    &[3, 11, 19, 27, 35, 43, 51, 59],
    &[4, 12, 20, 28, 36, 44, 52, 60],
    &[5, 13, 21, 29, 37, 45, 53, 61],
    &[6, 14, 22, 30, 38, 46, 54, 62],
    &[7, 15, 23, 31, 39, 47, 55, 63],
];


// **** Large lookup tables for every position of the board ****
// TODO: Potentially find different solution, not absolutely necessary
pub const POSITIVE_DIAGONAL_LOOKUP: [&[usize]; 64] = [
    P_DIAGONALS[0],
    P_DIAGONALS[1],
    P_DIAGONALS[2],
    P_DIAGONALS[3],
    P_DIAGONALS[4],
    P_DIAGONALS[5],
    P_DIAGONALS[6],
    P_DIAGONALS[7],
    P_DIAGONALS[1],
    P_DIAGONALS[2],
    P_DIAGONALS[3],
    P_DIAGONALS[4],
    P_DIAGONALS[5],
    P_DIAGONALS[6],
    P_DIAGONALS[7],
    P_DIAGONALS[8],
    P_DIAGONALS[2],
    P_DIAGONALS[3],
    P_DIAGONALS[4],
    P_DIAGONALS[5],
    P_DIAGONALS[6],
    P_DIAGONALS[7],
    P_DIAGONALS[8],
    P_DIAGONALS[9],
    P_DIAGONALS[3],
    P_DIAGONALS[4],
    P_DIAGONALS[5],
    P_DIAGONALS[6],
    P_DIAGONALS[7],
    P_DIAGONALS[8],
    P_DIAGONALS[9],
    P_DIAGONALS[10],
    P_DIAGONALS[4],
    P_DIAGONALS[5],
    P_DIAGONALS[6],
    P_DIAGONALS[7],
    P_DIAGONALS[8],
    P_DIAGONALS[9],
    P_DIAGONALS[10],
    P_DIAGONALS[11],
    P_DIAGONALS[5],
    P_DIAGONALS[6],
    P_DIAGONALS[7],
    P_DIAGONALS[8],
    P_DIAGONALS[9],
    P_DIAGONALS[10],
    P_DIAGONALS[11],
    P_DIAGONALS[12],
    P_DIAGONALS[6],
    P_DIAGONALS[7],
    P_DIAGONALS[8],
    P_DIAGONALS[9],
    P_DIAGONALS[10],
    P_DIAGONALS[11],
    P_DIAGONALS[12],
    P_DIAGONALS[13],
    P_DIAGONALS[7],
    P_DIAGONALS[8],
    P_DIAGONALS[9],
    P_DIAGONALS[10],
    P_DIAGONALS[11],
    P_DIAGONALS[12],
    P_DIAGONALS[13],
    P_DIAGONALS[14],
];

pub const NEGATIVE_DIAGONAL_LOOKUP: [&[usize]; 64] = [
    N_DIAGONALS[7], 
    N_DIAGONALS[8], 
    N_DIAGONALS[9], 
    N_DIAGONALS[10], 
    N_DIAGONALS[11],
    N_DIAGONALS[12],
    N_DIAGONALS[13], 
    N_DIAGONALS[14],
    N_DIAGONALS[6],
    N_DIAGONALS[7],
    N_DIAGONALS[8],
    N_DIAGONALS[9], 
    N_DIAGONALS[10], 
    N_DIAGONALS[11], 
    N_DIAGONALS[12], 
    N_DIAGONALS[13],
    N_DIAGONALS[5],
    N_DIAGONALS[6],
    N_DIAGONALS[7],
    N_DIAGONALS[8],
    N_DIAGONALS[9], 
    N_DIAGONALS[10], 
    N_DIAGONALS[11], 
    N_DIAGONALS[12],
    N_DIAGONALS[4],
    N_DIAGONALS[5],
    N_DIAGONALS[6],
    N_DIAGONALS[7],
    N_DIAGONALS[8],
    N_DIAGONALS[9], 
    N_DIAGONALS[10], 
    N_DIAGONALS[11],
    N_DIAGONALS[3],
    N_DIAGONALS[4],
    N_DIAGONALS[5],
    N_DIAGONALS[6],
    N_DIAGONALS[7],
    N_DIAGONALS[8],
    N_DIAGONALS[9], 
    N_DIAGONALS[10],
    N_DIAGONALS[2],
    N_DIAGONALS[3],
    N_DIAGONALS[4],
    N_DIAGONALS[5],
    N_DIAGONALS[6],
    N_DIAGONALS[7],
    N_DIAGONALS[8],
    N_DIAGONALS[9],
    N_DIAGONALS[1],
    N_DIAGONALS[2],
    N_DIAGONALS[3],
    N_DIAGONALS[4],
    N_DIAGONALS[5],
    N_DIAGONALS[6],
    N_DIAGONALS[7],
    N_DIAGONALS[8],
    N_DIAGONALS[0],
    N_DIAGONALS[1],
    N_DIAGONALS[2],
    N_DIAGONALS[3],
    N_DIAGONALS[4],
    N_DIAGONALS[5],
    N_DIAGONALS[6],
    N_DIAGONALS[7],
];

pub const ROW_LOOKUP: [&[usize]; 64] = [
    ROWS[0],
    ROWS[0],
    ROWS[0],
    ROWS[0],
    ROWS[0],
    ROWS[0],
    ROWS[0],
    ROWS[0],
    ROWS[1],
    ROWS[1],
    ROWS[1],
    ROWS[1],
    ROWS[1],
    ROWS[1],
    ROWS[1],
    ROWS[1],
    ROWS[2],
    ROWS[2],
    ROWS[2],
    ROWS[2],
    ROWS[2],
    ROWS[2],
    ROWS[2],
    ROWS[2],
    ROWS[3],
    ROWS[3],
    ROWS[3],
    ROWS[3],
    ROWS[3],
    ROWS[3],
    ROWS[3],
    ROWS[3],
    ROWS[4],
    ROWS[4],
    ROWS[4],
    ROWS[4],
    ROWS[4],
    ROWS[4],
    ROWS[4],
    ROWS[4],
    ROWS[5],
    ROWS[5],
    ROWS[5],
    ROWS[5],
    ROWS[5],
    ROWS[5],
    ROWS[5],
    ROWS[5],
    ROWS[6],
    ROWS[6],
    ROWS[6],
    ROWS[6],
    ROWS[6],
    ROWS[6],
    ROWS[6],
    ROWS[6],
    ROWS[7],
    ROWS[7],
    ROWS[7],
    ROWS[7],
    ROWS[7],
    ROWS[7],
    ROWS[7],
    ROWS[7],
];

pub const COLUMN_LOOKUP: [&[usize]; 64] = [
    COLUMNS[0],
    COLUMNS[1],
    COLUMNS[2],
    COLUMNS[3],
    COLUMNS[4],
    COLUMNS[5],
    COLUMNS[6],
    COLUMNS[7],
    COLUMNS[0],
    COLUMNS[1],
    COLUMNS[2],
    COLUMNS[3],
    COLUMNS[4],
    COLUMNS[5],
    COLUMNS[6],
    COLUMNS[7],
    COLUMNS[0],
    COLUMNS[1],
    COLUMNS[2],
    COLUMNS[3],
    COLUMNS[4],
    COLUMNS[5],
    COLUMNS[6],
    COLUMNS[7],
    COLUMNS[0],
    COLUMNS[1],
    COLUMNS[2],
    COLUMNS[3],
    COLUMNS[4],
    COLUMNS[5],
    COLUMNS[6],
    COLUMNS[7],
    COLUMNS[0],
    COLUMNS[1],
    COLUMNS[2],
    COLUMNS[3],
    COLUMNS[4],
    COLUMNS[5],
    COLUMNS[6],
    COLUMNS[7],
    COLUMNS[0],
    COLUMNS[1],
    COLUMNS[2],
    COLUMNS[3],
    COLUMNS[4],
    COLUMNS[5],
    COLUMNS[6],
    COLUMNS[7],
    COLUMNS[0],
    COLUMNS[1],
    COLUMNS[2],
    COLUMNS[3],
    COLUMNS[4],
    COLUMNS[5],
    COLUMNS[6],
    COLUMNS[7],
    COLUMNS[0],
    COLUMNS[1],
    COLUMNS[2],
    COLUMNS[3],
    COLUMNS[4],
    COLUMNS[5],
    COLUMNS[6],
    COLUMNS[7],
];