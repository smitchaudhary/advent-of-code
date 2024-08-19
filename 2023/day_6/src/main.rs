use std::process;

fn main() {
    let file_name = "input.txt".to_string();

    let part_1_answer = match day_6::race::part_1(&file_name) {
        Ok(num_ways) => num_ways,
        Err(err) => {
            eprintln!("Failed due to {}", err);
            process::exit(0);
        }
    };

    println!("The product of number of ways to win is {}", part_1_answer);

    let part_2_answer = match day_6::race::part_2(&file_name) {
        Ok(num_ways) => num_ways,
        Err(err) => {
            eprintln!("Failed due to {}", err);
            process::exit(1);
        }
    };

    println!("Total number of ways to win in part 2: {}", part_2_answer);
}
