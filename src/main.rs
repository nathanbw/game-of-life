extern crate rand;
use rand::Rng;

extern crate ncurses;
use ncurses::*;

fn main() {
    initscr();

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr, &mut max_y, &mut max_x);

    // TODO Validate max_x, max_y
    let width:           usize        = ((max_x / 3) - 1) as usize;
    let height:          usize        = (max_y - 2) as usize;
    let num_generations: u32           = 500;
    // board: the canonical Vec representing the state of our cells
    let mut board:       Vec<Vec<u8>> = vec![vec![0; width]; height];
    // tmp_board: Temporary board used to calculate intermediate states before
    // writing them to board.
    let mut tmp_board:   Vec<Vec<u8>> = vec![vec![0; width]; height];

    populate_board_random(&mut board, 20);

    for cur_generation in 0..(num_generations + 1) {
        print_board(&board);
        printw(&format!("Cur Gen: {}", cur_generation));
        refresh();
        std::thread::sleep(std::time::Duration::from_millis(20));
        advance_generation(&mut board, &mut tmp_board);
    }
    // getch(); // TODO "Press any key to quit" without leaving characters on the terminal
    endwin();
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

/// Prints the given board to stdscr
/// Assumes board has width smaller than 100
/// TODO: Pass in an ncurses window and update that instead of directly using stdscr
fn print_board(board: &Vec<Vec<u8>>) {
    let mut screen_row = 0;
    let screen_col = 0;
    wmove(stdscr, screen_row, screen_col);
    for cell_num in 0..(board[0].len() + 1) {
        printw(&format!("{:3}", cell_num));
    }
    screen_row += 1;
    wmove(stdscr, screen_row, 0);
    let mut row_num: u8 = 1;
    for row in board {
        printw(&format!("{:3}", row_num));
        row_num = row_num + 1;
        for cell in row {
            if *cell == 0 {
                printw("   ");
            } else {
                printw("  +");
            }
        }
        screen_row += 1;
        wmove(stdscr, screen_row, 0);
    }
}
