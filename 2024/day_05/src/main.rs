use day_05::solutions::{middle_of_correct_update, middle_of_incorrect_update};

fn main() {
    let filename = "input.txt".to_string();

    let part_1_answer = middle_of_correct_update(&filename);

    println!("The middle of the updates in correct updates is {part_1_answer}");

    let after_correction = middle_of_incorrect_update(&filename);

    println!(
        "The middle of the updates in incorrect updates is {}",
        after_correction - part_1_answer
    );
}
