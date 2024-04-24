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

struct Board {
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

fn handle_move(mv: Move, player_position: &mut Loc) -> bool {
    match mv {
        Move::Up => {
            if player_position.row > 1 {
                println!("You moved up!");
                player_position.row -= 1;
                return true;
            }
        }
        Move::Down => {
            if player_position.row < 10 {
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
            if player_position.col < 10 {
                println!("You moved right!");
                player_position.col += 1;
                return true;
            }
        }
    }
    false
}

fn main() {
    // Initializing the board with empty fields
    // + player's position outside of the board, for changing later
    let mut board = Board {
        field: vec![vec![Field::Empty; 12]; 12],
        player_position: Loc { row: 0, col: 0 },
        apples_positions: vec![],
    };

    let mut rng = rand::thread_rng();
    // Filling the apples positions randomly
    while board.apples_positions.len() < 10 {
        let row = rng.gen_range(1..10);
        let col = rng.gen_range(1..10);
        let apple_loc = Loc { row, col };
        // Making sure the apple doesn't land on another apple and is not out of the board
        let valid_place_for_apple = !board.apples_positions.contains(&apple_loc)
            && apple_loc.row != 0
            && apple_loc.row != 11
            && apple_loc.col != 0
            && apple_loc.col != 11;
        if valid_place_for_apple {
            board.apples_positions.push(apple_loc);
        }
    }
    // Generating a player_position, which is not already taken by an apple
    loop {
        board.player_position = Loc {
            row: rng.gen_range(1..=10),
            col: rng.gen_range(1..=10),
        };
        if !board.apples_positions.contains(&board.player_position) {
            break;
        }
    }

    loop {
        // Updating the board's state based on the player's position and apple positions
        for i in 0..12 {
            for j in 0..12 {
                let current_loc = Loc { row: i, col: j };
                if board.player_position == current_loc {
                    board.field[i][j] = Field::Player;
                } else if i == 0 || i == 11 || j == 0 || j == 11 {
                    board.field[i][j] = Field::Wall;
                } else if board.apples_positions.contains(&current_loc) {
                    board.field[i][j] = Field::Apple;
                } else {
                    board.field[i][j] = Field::Empty;
                }
            }
        }

        // Printing the board
        for row in &board.field {
            for cell in row {
                print!("{}", cell.to_string());
            }
            println!();
        }

        loop {
            let mut player_move: String = String::new();
            // Getting user's move
            io::stdin()
                .read_line(&mut player_move)
                .expect("Failed to read line");

            match convert_input_to_move_enum(player_move) {
                Some(mv) => {
                    if handle_move(mv, &mut board.player_position) {
                        break;
                    }
                }
                None => {
                    println!("To move, use WSAD!");
                }
            }
        }
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

        if board.apples_positions.is_empty() {
            println!("\nY O U  W O N !    Y O U  W O N !    Y O U  W O N !    Y O U  W O N !    Y O U  W O N !\n");
            main();
        }
    }
}
