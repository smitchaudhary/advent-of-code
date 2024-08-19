use std::process;

fn main() {
    let file_name = "input.txt".to_string();

    // let part_1_answer = match day_8::navigation::run(&file_name) {
    //     Ok(num_steps) => num_steps,
    //     Err(err) => {
    //         eprintln!("Failed due to: {}", err);
    //         process::exit(1);
    //     }
    // };

    // println!("The number of steps is: {}", part_1_answer);

    let part_2_answer = match day_8::navigation::simultaneous_run(&file_name) {
        Ok(num_steps) => num_steps,
        Err(err) => {
            println!("Failed due to: {}", err);
            process::exit(1);
        }
    };

    println!(
        "If we do simultaneous search, number of steps = {}",
        part_2_answer
    );
}
