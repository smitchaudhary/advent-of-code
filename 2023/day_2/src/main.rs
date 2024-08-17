use std::process;

fn main() {
    let file_name = "input.txt".to_string();

    let part_1_answer = match day_2::cubes::part_1(&file_name) {
        Ok(total) => total,
        Err(err) => {
            eprintln!("Failed due to {}", err);
            process::exit(1);
        }
    };

    println!("The total sum of valid game numbers is {}", part_1_answer);

    let part_2_answer = match day_2::cubes::part_2(&file_name) {
        Ok(power) => power,
        Err(err) => {
            eprintln!("Failed due to {}", err);
            process::exit(1);
        }
    };

    println!("The total power is {}", part_2_answer);
}
