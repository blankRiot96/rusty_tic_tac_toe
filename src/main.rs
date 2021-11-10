use std::io;
use rand::Rng;
use ansi_term::Style;
use ansi_term::Colour;


fn check_win(board: &[[i32; 3]; 3]) -> bool {
    let mut win = false;
    for row in board {
        // Checking win for any Row
        if row[0] != 0 {
            if row[0] == row[1]  {
                if row[1] == row[2] {
                    win = true;
                    println!("{}{}{}", row[0], row[1], row[2]);
                    break;
                }
            } 
        }
        for column in row {
            let column: usize = column.to_owned() as usize;
            // Checking win for any column
            if board[0][column] != 0  {
                if board[0][column] == board[1][column] {
                    if board[1][column] == board[2][column] {
                        win = true;
                        println!("{}{}{}", board[0][column], board[1][column], board[2][column]);
                        break
                    }
                }
            }
        }
    }

    // Checking win for both diagonals
    // Top left to Bottom right
    if board[0][0] != 0 {
        if board[0][0] == board[1][1] {
            if board[1][1] == board[2][2] {
                win = true;
            }
        }
    }

    // Top Right to Bottom left
    if board[0][2] != 0 {
        if board[0][2] == board[1][1] {
            if board[1][1] == board[2][0] {
                win = true;
            }
        }
    }

    win
}


fn draw_results(results: &Vec<String>) {
    // Opening Results
    println!("{1}\n{0}", 
             Style::new().bold().paint("- ".repeat(6)),
             Style::new().bold().paint("Results:"));


    if results.len() > 0 {
        for result in results {
            println!("{}", result);
        }
    } else {
        println!("{}", Style::new().bold().paint("None"));
    }
    
    // Closing results
    println!("{}",
             Style::new().bold().paint("- ".repeat(6)));

}


fn get_position() -> String {
        println!("Enter a position: ");

        // Creating a mutable empty string to hold the `input` value
        let mut pos = String::new();

        // Getting the input by reading the line
        io::stdin()
            .read_line(&mut pos)
            .expect("Failed to read guess");

        // Removing "\n" and converting back to string 
        pos = pos.trim().to_string();
        pos
        
}


fn replace_position(mut board: [[i32; 3]; 3], n: i32, pos: String) ->  ([[i32; 3]; 3], bool, bool) {
    /* Pos Will be a value 
    like "1c" where 1 will be the row and "c" will be the column
        a   b   c
      +---+---+---+
    1 │   │   │   │
      +---+---+---+
    2 │   │   │   │
      +---+---+---+
    3 │   │   │   │
      +---+---+---+
    */
    // Example: Spliting "1c" into 1 and "c"
    let row_s = &pos[0..1];
    let column_s = &pos[1..2];

    let mut row: usize = row_s.trim().parse().expect("Please type a number!");

    // Error variables
    let mut no_errors = true;
    let mut results: Vec<String> = Vec::new();

    // Error handling
    if ![1, 2, 3].contains(&row) {
        results.push(format!("{}", Colour::Red.paint("Your Row must be a value from 1 to 3!")));
        no_errors = false;
    }
    if !["a", "b", "c"].contains(&column_s) {
        results.push(format!("{}", Colour::Red.paint("Your Column must be a value in [a, b, c]!")));
        no_errors = false;
    }


    // Decreasing Row to make it a valid coordinate
    row -= 1;
    
    

    let column = match column_s {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        _ => 0
    };


    if no_errors {
        if board[row][column] == 0 {
            board[row][column] = n;
        } else {
            results.push(format!("Position {0} {1}",
                         pos,
                         Colour::Red.paint("has been taken!")));
            no_errors = false;
        }
    }


    // Checking for win
    let win = check_win(&board);
    if win {
        results.push(format!("{}",
                     Colour::Green.paint(format!("Player {} has won!", n))));
    }

    // Drawing results
    draw_results(&results);

    (board, win, no_errors)
}

fn draw_board(board: &[[i32; 3]; 3]) {
    let (_a, b, _c, d) = ("┌", "-", "┐", "│");
    println!("    a   b   c");

    let mut current_row = 1;
    let mut once = true;
    for row in board {
        println!("  +{0}+{0}+{0}+", b.repeat(3));
        for column in row {
            if once {
                print!("{} ", current_row);
                once = false;
            }
            print!("{}", d);
            if column == &0 {
                print!("   ");
            } else if column == &1 {
                print!(" X ");
            } else if column == &2 {
                print!(" O ");
            }
        }
        current_row += 1;
        once = true;
        print!("{}", d);
        println!("");
    }
    println!("  +{0}+{0}+{0}+", b.repeat(3));
}

fn main() {
    let mut board: [[i32; 3]; 3]  = [[0, 0, 0],
                                     [0, 0, 0],
                                     [0, 0, 0]];
    let mut player = true;

    let mode = loop {
        println!("What mode do you want to play?\n(a) single player\n(b) multiplayer");

        // Getting the input by reading the line
        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Invalid");

        if ["a\r\n".to_string(), "b\r\n".to_string()].contains(&x) {
            break x;
        } else {
            println!("Invalid entry.\nHint: Must be 'a' or 'b'");
        }
    };


    if mode == "a\r\n".to_string() {
        loop {
            let win = loop {
                // Draw the board
                draw_board(&board);

                println!("Its player 1s turn!");

                // Replace positions
                let values = replace_position(board, 1, get_position());
                board = values.0;
                let win = values.1;
                let no_errors = values.2;

                if no_errors {
                    break win;
                }
            };


            if win {
                draw_board(&board);
                break;
            }
            
            let win = loop {
                println!("Its player 2s turn!");
                // Play Player 2's move
                let row = rand::thread_rng().gen_range(1..4);
                let col = rand::thread_rng().gen_range(1..4);
                let col = match col {
                    1 => "a",
                    2 => "b",
                    3 => "c",
                    _ => "a"
                };
    
                let pos = format!("{}{}", row, col);
                println!("{}", pos);
                let values = replace_position(board, 2, pos);
                board = values.0;
                let win = values.1;
                let no_errors = values.2;

                if no_errors {
                    break win;
                }
            };

            if win {
                draw_board(&board);
                break
            }
        }
    } else if mode == "b\r\n".to_string() {
            loop {
                // Draw the board
                draw_board(&board);

                // Switch player turns
                player = !player;
                let n = match player {
                    false => 1,
                    true => 2
                };

                println!("Its player {0}s turn!", n);

                // Replace positions
                let values = replace_position(board, n, get_position());
                board = values.0;
                let win = values.1;
                let no_errors = values.2;

                if !no_errors {
                    player = !player;
                }

                if win {
                    draw_board(&board);
                    break
                }
            }
        }
}
