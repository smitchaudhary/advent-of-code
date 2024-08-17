use std::process;

fn main() {
    let file_name = "input.txt".to_string();

    let part_1_answer = match day_3::map::part_1(&file_name) {
        Ok(sum) => sum,
        Err(err) => {
            eprintln!("Failed due to {}", err);
            process::exit(1);
        }
    };

    println!("The total sum of part number is {}", part_1_answer);

    let part_2_answer = match day_3::map::part_2(&file_name) {
        Ok(sum) => sum,
        Err(err) => {
            eprintln!("Failed due to {}", err);
            process::exit(1);
        }
    };

    println!("The total sum of gear rations is {}", part_2_answer);
}
