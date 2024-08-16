use std::process;

fn main() {
    let file_name = "input.txt".to_string();

    let part_1_answer = match day_1::part_1::run(&file_name) {
        Ok(total) => total,
        Err(err) => {
            eprintln!("Failed due to {}", err);
            process::exit(1);
        }
    };

    println!(
        "The total calibration value for part 1 is {}",
        part_1_answer
    );

    let part_2_answer = match day_1::part_2::run(&file_name) {
        Ok(total) => total,
        Err(err) => {
            eprintln!("Failed due to {}", err);
            process::exit(1);
        }
    };

    println!(
        "The total calibration value for part 2 is {}",
        part_2_answer
    )
}
