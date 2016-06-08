extern crate rand;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let width:     usize = 10; // FUTURE: can get this value from user
    let height:    usize = 10; // FUTURE: can get this value from user
    let mut board: Vec<Vec<u8>> = vec![vec![0; height]; width];
    populate_board_random(&mut board, 20); // FUTURE: can get this value from user
    print_board(&board);
}

fn print_board(board: &Vec<Vec<u8>>) {
    for row in board {
        for cell in row {
            if *cell == 0 {
                print!("  ");
            } else {
                print!("+ ");
            }
        }
        println!("");
    }
}

fn populate_board_random(board: &mut Vec<Vec<u8>>, percent_alive: u8) {
    for row in board {
        for cell in row {
            let die_value: u8 = rand::thread_rng().gen_range(1, 101);
            match percent_alive.cmp(&die_value) {
                Ordering::Greater => *cell = 1,
                _                 => (),
            }
        }
    }
}
