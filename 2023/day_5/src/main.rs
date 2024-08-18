use std::process;

fn main() {
    let file_name = "input.txt".to_string();

    let part_1_answer = match day_5::seeds::part_1(&file_name) {
        Ok(lowest_location) => lowest_location,
        Err(err) => {
            eprintln!("Failed due to {}", err);
            process::exit(1);
        }
    };

    println!("The closest location is {}", part_1_answer);

    let part_2_answer = match day_5::seeds::part_2_alt(&file_name) {
        Ok(lowest_location) => lowest_location,
        Err(err) => {
            eprintln!("Failed due to {}", err);
            process::exit(1);
        }
    };

    println!("The closest location in part 2 is {}", part_2_answer);
}
