use rand::Rng;
use std::collections::HashSet;
use std::io;

#[derive(Clone, Debug, PartialEq)]
enum Field {
    Empty(String),
    Apple(String),
    Wall(String),
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

fn handle_move(mv: Move, player_position: &mut i32) -> bool {
    match mv {
        Move::Up => {
            if (*player_position - 12) > 12 {
                println!("You moved up!");
                *player_position -= 12;
                return true;
            }
        }
        Move::Down => {
            if *player_position + 12 < 132 {
                println!("You moved down!");
                *player_position += 12;
                return true;
            }
        }
        Move::Left => {
            if (*player_position - 1) % 12 > 1 {
                println!("You moved left!");
                *player_position -= 1;
                return true;
            }
        }
        Move::Right => {
            if (*player_position + 1) % 12 != 0 {
                println!("You moved right!");
                *player_position += 1;
                return true;
            }
        }
    }
    false
}

impl Field {
    fn empty() -> Self {
        Field::Empty(String::from(". "))
    }
    fn apple() -> Self {
        Field::Apple(String::from("* "))
    }
    fn wall() -> Self {
        Field::Wall(String::from("# "))
    }
}

fn main() {
    let mut board: Vec<Vec<Field>> = vec![vec![Field::Empty(String::from(". ")); 12]; 12];

    let mut rng = rand::thread_rng();
    let ranges_for_board = [
        14..=23,
        26..=35,
        38..=47,
        50..=59,
        62..=71,
        74..=83,
        86..=95,
        98..=107,
        110..=119,
        124..=131,
    ];
    let mut apples_positions = HashSet::new();
    // Filling the apples positions HashSet
    while apples_positions.len() < 10 {
        let selected_range_index = rng.gen_range(0..ranges_for_board.len());
        let selected_range = &ranges_for_board[selected_range_index];
        apples_positions.insert(rng.gen_range(selected_range.clone()));
    }
    // Generating a player_position, which is not already taken by an apple
    let selected_range_index = rng.gen_range(0..ranges_for_board.len());
    let selected_range = &ranges_for_board[selected_range_index];
    let mut player_position = rng.gen_range(selected_range.clone());

    let mut apples_left = apples_positions.len();

    loop {
        // Variable for tracking elements position on the board
        let mut position_counter = 0;
        for i in 0..12 {
            for j in 0..12 {
                position_counter += 1;
                if i == 0 || i == 11 || j == 0 || j == 11 {
                    board[i][j] = Field::wall();
                } else if position_counter == player_position {
                    board[i][j] = Field::Empty(String::from("@"))
                } else if apples_positions.contains(&position_counter) {
                    board[i][j] = Field::apple();
                } else {
                    board[i][j] = Field::empty();
                }
            }
        }

        // Print the board for debugging
        for row in &board {
            for cell in row {
                match cell {
                    Field::Empty(contents) => print!("{:<2}", contents),
                    Field::Apple(contents) => print!("{:<2}", contents),
                    Field::Wall(contents) => print!("{:<2}", contents),
                }
            }
            println!();
        }

        // board[player_position / 12][player_position % 12] != Field::Wall(String::from("# "))

        loop {
            let mut player_move: String = String::new();

            io::stdin()
                .read_line(&mut player_move)
                .expect("Failed to read line");

            match convert_input_to_move_enum(player_move) {
                Some(mv) => {
                    if handle_move(mv, &mut player_position) {
                        break;
                    }
                }
                None => {
                    println!("To move, use WSAD!");
                }
            }
        }

        // Remove the apple if the player moved onto it
        if apples_positions.remove(&player_position) {
            apples_left -= 1;
            println!("You ate an apple!");
        }

        if apples_left == 0 {
            println!("\nY O U  W O N !    Y O U  W O N !    Y O U  W O N !    Y O U  W O N !    Y O U  W O N !\n");
            main();
        }
    }
}
