use std::process;

use day_7::cards::ScoringMode;

fn main() {
    let file_name = "input.txt".to_string();

    let part_1_answer = match day_7::cards::run(&file_name, ScoringMode::Normal) {
        Ok(winnings) => winnings,
        Err(err) => {
            eprintln!("Failed due to: {}", err);
            process::exit(1);
        }
    };

    println!("The winnings: {}", part_1_answer);

    let part_2_answer = match day_7::cards::run(&file_name, ScoringMode::WithJoker) {
        Ok(winnings) => winnings,
        Err(err) => {
            eprintln!("Failed due to: {}", err);
            process::exit(1);
        }
    };

    println!("The winnings with joker: {}", part_2_answer);
}
