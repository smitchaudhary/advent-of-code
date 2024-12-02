fn main() {
    let file_name = "input.txt".to_string();

    let part_1_answer = day_14::part_1::run(&file_name);

    println!("The total load on north beam is: {}", part_1_answer);
}
