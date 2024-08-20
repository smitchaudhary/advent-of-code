fn main() {
    let file_name = "input.txt".to_string();

    let part_1_answer = day_11::galaxy::run(&file_name, 2);

    println!("Total distance is: {}", part_1_answer);

    let part_2_answer = day_11::galaxy::run(&file_name, 1_000_000);

    println!("Total distance is: {}", part_2_answer);
}
