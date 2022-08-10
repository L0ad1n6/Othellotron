mod bot;
mod game;
mod human;

use crate::game::Board;

fn main() {
    let mut board = Board::new();
    println!("{board}");
    
    // let mut i = 0;
    // loop {
    //     println!("Ply {}: {}", i, crate::bot::moves::_ply_move_counter_test(&board, i, true));
    //     i += 1;
    // }

    loop {
        human::play(&mut board, true);
        println!("{board}");
        bot::play(&mut board, false, 7);
        println!("{board}");
    }

}
