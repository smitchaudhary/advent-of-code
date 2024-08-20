fn main() {
    let file_name = "input.txt".to_string();

    let part_1_answer = day_9::part_1::run(&file_name);

    println!("The answer is: {}", part_1_answer);

    let part_2_answer = day_9::part_2::run(&file_name);

    println!("The answer to part 2 is {}", part_2_answer);
}
