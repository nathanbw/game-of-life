extern crate rand;
use rand::Rng;

fn main() {
    // FUTURE: can get width, height, population density, and num_generations from user:
    // width and height hould be greater than 0 and less than 100
    let width:           usize        = 15;
    let height:          usize        = 15;
    let num_generations: u8           = 10;
    // board: the canonical Vec representing the state of our cells
    let mut board:       Vec<Vec<u8>> = vec![vec![0; height]; width];
    // tmp_board: Temporary board used to calculate intermediate states before
    // writing them to board.
    let mut tmp_board:   Vec<Vec<u8>> = vec![vec![0; height]; width];

    populate_board_random(&mut board, 20);
    hacky_and_bad_screen_clear();

    print_board(&board);
    println!("Cur Gen: 0 (Initial State)");
    std::thread::sleep(std::time::Duration::new(1, 0));

    for cur_generation in 1..(num_generations + 1) {
        advance_generation(&mut board, &mut tmp_board);
        std::thread::sleep(std::time::Duration::new(1, 0));
        hacky_and_bad_screen_clear();
        print_board(&board);
        println!("Cur Gen: {}", cur_generation);
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

/// Advances the given board by one generation of the Game of Life
/// Assumes board has dimensions greater than [0][0] and smaller than [100][100]
fn advance_generation(board: &mut Vec<Vec<u8>>, tmp_board: &mut Vec<Vec<u8>>) {
    for row_num in 0..board.len() {
        for cell_num in 0..board[0].len() {
            tmp_board[row_num][cell_num] = calculate_next_state(&board, row_num, cell_num);
        }
    }
    for row_num in 0..board.len() {
        for cell_num in 0..board[0].len() {
            board[row_num][cell_num] = tmp_board[row_num][cell_num];
        }
    }
}

/// Given a board and the coordinates of the cell, returns the value this cell will hold
/// in the next generation
fn calculate_next_state(board: &Vec<Vec<u8>>, row_num: usize, cell_num: usize) -> u8 {
    let mut num_neighbors_alive: u8 = 0;

    // check top left:
    num_neighbors_alive += {
        if row_num != 0 && cell_num != 0 {
            board[row_num - 1][cell_num - 1]
        } else {
            0
        }
    };
    // check top middle:
    num_neighbors_alive += {
        if row_num != 0 {
            board[row_num - 1][cell_num]
        } else {
            0
        }
    };
    // check top right:
    num_neighbors_alive += {
        if row_num != 0 && cell_num != (board[0].len() - 1) {
            board[row_num - 1][cell_num + 1]
        } else {
            0
        }
    };
    // check left:
    num_neighbors_alive += {
        if cell_num != 0 {
            board[row_num][cell_num - 1]
        } else {
            0
        }
    };
    // check right:
    num_neighbors_alive += {
        if cell_num != (board[0].len() - 1) {
            board[row_num][cell_num + 1]
        } else {
            0
        }
    };
    // check bottom left:
    num_neighbors_alive += {
        if row_num != (board.len() - 1) && cell_num != 0 {
            board[row_num + 1][cell_num - 1]
        } else {
            0
        }
    };
    // check bottom right
    num_neighbors_alive += {
        if row_num != (board.len() - 1) && cell_num != (board[0].len() - 1) {
            board[row_num + 1][cell_num + 1]
        } else {
            0
        }
    };
    // check bottom middle
    num_neighbors_alive += {
        if row_num != (board.len() - 1) {
            board[row_num + 1][cell_num]
        } else {
            0
        }
    };

    if num_neighbors_alive < 2 || num_neighbors_alive > 3 {
        0
    } else if num_neighbors_alive == 2 {
        board[row_num][cell_num]
    } else {
        1
    }
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

fn hacky_and_bad_screen_clear()
{
    if !std::process::Command::new("clear").status().unwrap().success() {
        assert!(false);
    }
}
