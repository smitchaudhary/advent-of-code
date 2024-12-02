fn main() {
    let file_name = "input.txt".to_string();

    let part_1_answer = day_13::lava::run(&file_name);

    println!("The summarised number is {}", part_1_answer);
}
