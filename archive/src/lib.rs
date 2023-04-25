use std::io::Write;
use std::thread;
use std::time::Duration;

const WIDTH: usize = 50;
const HEIGHT: usize = 20;
const RECURSION_DEPTH: usize = 2;

#[derive(Clone)]
enum Cell {
    Dead,
    Alive,
    Board(Vec<Vec<Cell>>),
}

fn main() {
    let mut board = create_board(RECURSION_DEPTH);
    initialize_board(&mut board);
    loop {
        print_board(&board);
        board = evolve(&board);
        thread::sleep(Duration::from_millis(200));
    }
}

fn create_board(depth: usize) -> Vec<Vec<Cell>> {
    if depth == 0 {
        vec![vec![Cell::Dead; WIDTH]; HEIGHT]
    } else {
        vec![vec![create_board(depth - 1); WIDTH]; HEIGHT]
    }
}

fn initialize_board(board: &mut Vec<Vec<Cell>>) {
    // Add some initial live cells here, for example:
    board[5][5] = Cell::Alive;
    board[5][6] = Cell::Alive;
    board[6][5] = Cell::Alive;
    board[6][6] = Cell::Alive;

    // Add a smaller board inside a cell
    if let Cell::Board(ref mut inner_board) = board[10][10] {
        inner_board[1][2] = Cell::Alive;
        inner_board[2][3] = Cell::Alive;
        inner_board[3][1] = Cell::Alive;
        inner_board[3][2] = Cell::Alive;
        inner_board[3][3] = Cell::Alive;
    }
}

fn print_board(board: &Vec<Vec<Cell>>) {
    print!("\x1B[2J\x1B[1;1H"); // Clear terminal and move cursor to top-left corner
    print_board_recursive(board, 0);
    std::io::stdout().flush().unwrap();
}

fn print_board_recursive(board: &Vec<Vec<Cell>>, depth: usize) {
    for row in board {
        for cell in row {
            match cell {
                Cell::Dead => print!("."),
                Cell::Alive => print!("O"),
                Cell::Board(inner_board) if depth < RECURSION_DEPTH => {
                    print!("(");
                    print_board_recursive(inner_board, depth + 1);
                    print!(")");
                }
                Cell::Board(_) => print!("#"),
            }
        }
        println!();
    }
}

fn evolve(board: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut new_board = board.clone();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let live_neighbors = count_live_neighbors(x, y, &board);
            new_board[y][x] = match (&board[y][x], live_neighbors) {
                (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                (Cell::Dead, 3) => Cell::Alive,
                (Cell::Board(inner_board), _) => {
                    let evolved_inner_board = evolve(inner_board);
                    Cell::Board(evolved_inner_board)
                }
                _ => Cell::Dead,
            };
        }
    }

    new_board
}

fn count_live_neighbors(x: usize, y: usize, board: &Vec<Vec<Cell>>) -> usize {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                match &board[ny as usize][nx as usize] {
                    Cell::Alive => count += 1,
                    _ => (),
                }
            }
        }
    }

    count
}
