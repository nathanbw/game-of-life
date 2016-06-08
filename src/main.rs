extern crate rand;
use rand::Rng;

fn main() {
    // FUTURE: can get width, height, population density, and num_generations from user:
    // width and height hould be greater than 0 and less than 100
    let width:           usize        = 20;
    let height:          usize        = 20;
    let num_generations: u8           = 100;
    let mut board:       Vec<Vec<u8>> = vec![vec![0; height]; width];
    populate_board_random(&mut board, 20);
    print_board(&board);
}

/// Prints the given board
/// Assumes board has dimensions greater than [0][0] and smaller than [100][100]
fn print_board(board: &Vec<Vec<u8>>) {
    for cell_num in 0..(board[0].len() + 1) {
        print!("{:3}", cell_num);
    }
    println!("");
    let mut row_num: u8 = 1;
    for row in board {
        print!("{:3}", row_num);
        row_num = row_num + 1;
        for cell in row {
            if *cell == 0 {
                print!("   ");
            } else {
                print!("  +");
            }
        }
        println!("");
    }
}

/// Populates the given board with a generation containing
/// percent_alive percent of live cells
fn populate_board_random(board: &mut Vec<Vec<u8>>, percent_alive: u8) {
    for row in board {
        for cell in row {
            let die_value: u8 = rand::thread_rng().gen_range(1, 101);
            if percent_alive >= die_value {
                *cell = 1;
            }
        }
    }
}
