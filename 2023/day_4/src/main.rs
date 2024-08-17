use std::process;

fn main() {
    let file_name = "input.txt".to_string();

    let part_1_answer = match day_4::scratch_cards::part_1(&file_name) {
        Ok(points) => points,
        Err(err) => {
            eprintln!("Failed due to {}", err);
            process::exit(1);
        }
    };

    println!("Total points = {}", part_1_answer);

    let part_2_answer = match day_4::scratch_cards::part_2(&file_name) {
        Ok(num_cards) => num_cards,
        Err(err) => {
            eprintln!("Failed due to {}", err);
            process::exit(1);
        }
    };

    println!("Total number of scratch cards = {}", part_2_answer);
}
