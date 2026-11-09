use rand::random_range;
use std::io::{self, Write};

fn get_random_choice() -> u8 {
    random_range(1..=3)
}

fn get_user_choice() -> u8 {
    return loop {
        print!("Enter your choice (Stone-1, Paper-2, Scissor-3, Quit-0): ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line!");
        let ch: u8 = match buf.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Please enter a valid number!");
                continue;
            }
        };
        if ch <= 3 {
            break ch;
        } else {
            eprintln!("Invalid choice!");
        }
    };
}

fn check_result(user_choice: u8, random_choice: u8) -> u8 {
    static WINNING_COMBINATIONS: [[u8; 2]; 3] = [[1, 3], [2, 1], [3, 2]];
    static LOSING_COMBINATIONS: [[u8; 2]; 3] = [[1, 2], [2, 3], [3, 1]];

    let choice_arr = [user_choice, random_choice];
    if WINNING_COMBINATIONS.contains(&choice_arr) {
        return 1;
    } else if LOSING_COMBINATIONS.contains(&choice_arr) {
        return 2;
    } else {
        return 0;
    }
}

fn get_choice_string(ch: u8) -> &'static str {
    if ch == 1 {
        return "STONE";
    } else if ch == 2 {
        return "PAPER";
    } else if ch == 3 {
        return "SCISSOR";
    } else {
        return "INVALID";
    }
}

fn get_result_string(res: u8) -> &'static str {
    if res == 1 {
        return "YOU WIN :)";
    } else if res == 2 {
        return "YOU LOSE :(";
    } else if res == 0 {
        return "ITS A TIE";
    } else {
        return "INVALID";
    }
}

fn get_score(res: u8) -> i32 {
    static mut SCORE: i32 = 0;
    unsafe {
        if res == 1 {
            SCORE += 1;
        } else if res == 2 {
            SCORE -= 1;
        }
        return SCORE;
    }
}

fn game_loop() {
    loop {
        let user_ch = get_user_choice();

        if user_ch == 0 {
            break;
        }

        let rand_ch = get_random_choice();
        let res = check_result(user_ch, rand_ch);

        println!("Your choice: {}", get_choice_string(user_ch));
        println!("Random choice: {}", get_choice_string(rand_ch));
        println!("Result: {}", get_result_string(res));
        println!("Your score: {}", get_score(res));
    }
}

fn main() {
    println!("Welcome to Stone, Paper, Scissor!");
    game_loop();
    println!("Quitting game, bye");
}
