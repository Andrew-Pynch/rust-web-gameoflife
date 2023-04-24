use std::io::Write;
use std::thread;
use std::time::Duration;

const WIDTH: usize = 50;
const HEIGHT: usize = 20;

fn main() {
    let mut board = vec![vec![false; WIDTH]; HEIGHT];
    initialize_board(&mut board);

    loop {
        print_board(&board);
        board = evolve(&board);
        thread::sleep(Duration::from_millis(200));
    }
}

fn initialize_board(board: &mut Vec<Vec<bool>>) {
    // Add a Glider
    board[1][2] = true;
    board[2][3] = true;
    board[3][1] = true;
    board[3][2] = true;
    board[3][3] = true;
}

fn print_board(board: &Vec<Vec<bool>>) {
    print!("\x1B[2J\x1B[1;1H"); // Clear terminal and move cursor to top-left corner
    for row in board {
        for &cell in row {
            let c = if cell { 'O' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
    std::io::stdout().flush().unwrap();
}

fn evolve(board: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_board = vec![vec![false; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let live_neighbors = count_live_neighbors(x, y, &board);
            new_board[y][x] = match (board[y][x], live_neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    new_board
}

fn count_live_neighbors(x: usize, y: usize, board: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                if board[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
    }

    count
}
