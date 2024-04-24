use rand::Rng;
use std::{io, usize};

#[derive(Clone, Debug, PartialEq)]
enum Field {
    Empty,
    Apple,
    Wall,
    Player,
}

impl Field {
    fn to_string(&self) -> String {
        match self {
            Field::Empty => ". ",
            Field::Apple => "* ",
            Field::Wall => "# ",
            Field::Player => "@ ",
        }
        .to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Loc {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy)]
struct Size {
    rows: usize,
    cols: usize,
}

struct Board {
    size: Size,
    field: Vec<Vec<Field>>,
    player_position: Loc,
    apples_positions: Vec<Loc>,
}

#[derive(Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn convert_input_to_move_enum(input: String) -> Option<Move> {
    match input.trim() {
        "w" => Some(Move::Up),
        "s" => Some(Move::Down),
        "a" => Some(Move::Left),
        "d" => Some(Move::Right),
        _ => None,
    }
}

fn handle_move(mv: Move, player_position: &mut Loc, rows: usize, cols: usize) -> bool {
    match mv {
        Move::Up => {
            if player_position.row > 1 {
                println!("You moved up!");
                player_position.row -= 1;
                return true;
            }
        }
        Move::Down => {
            if player_position.row < rows - 2 {
                println!("You moved down!");
                player_position.row += 1;
                return true;
            }
        }
        Move::Left => {
            if player_position.col > 1 {
                println!("You moved left!");
                player_position.col -= 1;
                return true;
            }
        }
        Move::Right => {
            if player_position.col < cols - 2 {
                println!("You moved right!");
                player_position.col += 1;
                return true;
            }
        }
    }
    false
}

fn get_board_size_from_user() -> Size {
    loop {
        println!("Enter the number of rows:");
        let mut rows = String::new();
        io::stdin()
            .read_line(&mut rows)
            .expect("Failed to read input");

        let rows: usize = match rows.trim().parse::<usize>() {
            Ok(num) if num >= 1 => num + 2,
            _ => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("Enter the number of columns:");
        let mut cols = String::new();
        io::stdin()
            .read_line(&mut cols)
            .expect("Failed to read input");

        let cols: usize = match cols.trim().parse::<usize>() {
            Ok(num) if num >= 1 => num + 2,
            _ => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if rows <= 3 && cols <= 3 {
            println!("Please put in different dimensions. Game on a 1x1 board won't be any fun :p");
            continue;
        }

        return Size { rows, cols };
    }
}

fn initialize_board(size: Size) -> Board {
    let board = Board {
        size,
        field: vec![vec![Field::Empty; size.cols]; size.rows],
        player_position: Loc { row: 0, col: 0 },
        apples_positions: vec![],
    };
    board
}

fn generate_apples_positions(board: &mut Board, rng: &mut rand::rngs::ThreadRng) {
    let rows = board.size.rows;
    let cols = board.size.cols;
    let board_size = (rows - 2) * (cols - 2);
    // usize doesn't support ceil(), so this conversions are needed
    // (board_size + 9) / 10 makes sure for boards like 5x5 there's 3 apples, but for 10x10 there's 10
    let percent_of_apples = (((board_size + 9) / 10) as f64).ceil() as usize;
    println!(
        "\nboard size: {}, number of apples: {}\n",
        board_size, percent_of_apples
    );

    while board.apples_positions.len() < percent_of_apples {
        let row = rng.gen_range(1..rows);
        let col = rng.gen_range(1..cols);
        let apple_loc = Loc { row, col };
        // Making sure the apple doesn't land on another apple and is not out of the board
        let valid_place_for_apple = !board.apples_positions.contains(&apple_loc)
            && apple_loc.row != 0
            && apple_loc.row != rows - 1
            && apple_loc.col != 0
            && apple_loc.col != cols - 1;
        if valid_place_for_apple {
            board.apples_positions.push(apple_loc);
        }
    }
}

fn generate_player_position(board: &mut Board, rng: &mut rand::rngs::ThreadRng) {
    let rows = board.size.rows;
    let cols = board.size.cols;

    loop {
        board.player_position = Loc {
            row: rng.gen_range(1..=rows - 2),
            col: rng.gen_range(1..=cols - 2),
        };
        if !board.apples_positions.contains(&board.player_position) {
            break;
        }
    }
}

fn update_board_state(board: &mut Board) {
    let rows = board.size.rows;
    let cols = board.size.cols;

    for i in 0..rows {
        for j in 0..cols {
            let current_loc = Loc { row: i, col: j };
            let should_be_wall = i == 0 || i == rows - 1 || j == 0 || j == cols - 1;

            if board.player_position == current_loc {
                board.field[i][j] = Field::Player;
            } else if should_be_wall {
                board.field[i][j] = Field::Wall;
            } else if board.apples_positions.contains(&current_loc) {
                board.field[i][j] = Field::Apple;
            } else {
                board.field[i][j] = Field::Empty;
            }
        }
    }
}

fn print_board(board: &Board) {
    for row in &board.field {
        for cell in row {
            print!("{}", cell.to_string());
        }
        println!();
    }
}

fn handle_player_moves(board: &mut Board) {
    let rows = board.size.rows;
    let cols = board.size.cols;

    loop {
        let mut player_move: String = String::new();
        // Getting user's move
        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");

        match convert_input_to_move_enum(player_move) {
            Some(mv) => {
                if handle_move(mv, &mut board.player_position, rows, cols) {
                    break;
                }
            }
            None => {
                println!("To move, use WSAD!");
            }
        }
    }
}

fn remove_eaten_apple(board: &mut Board) {
    // Remove the apple if the player moved onto it
    if let Some(index) = board
        .apples_positions
        // Creates an iterator fo apples_positions
        .iter()
        // Searches for the position of an element in the iterator
        // Closure takes &Loc and dereferences it, to compare its value to player_position
        .position(|loc| *loc == board.player_position)
    {
        board.apples_positions.remove(index);
        println!("You ate an apple!");
    }
}

fn player_won(board: &Board) -> bool {
    board.apples_positions.is_empty()
}

fn print_congrats_and_restart() {
    println!("\nY O U  W O N !    Y O U  W O N !    Y O U  W O N !    Y O U  W O N !    Y O U  W O N !\n");
    main();
}

fn main() {
    println!("Welcome to the apple eating game!");

    let size = get_board_size_from_user();
    let mut board = initialize_board(size);

    let mut rng = rand::thread_rng();
    generate_apples_positions(&mut board, &mut rng);
    generate_player_position(&mut board, &mut rng);

    loop {
        update_board_state(&mut board);
        print_board(&board);
        handle_player_moves(&mut board);
        remove_eaten_apple(&mut board);

        if player_won(&board) {
            print_congrats_and_restart();
        }
    }
}
