fn main() {
    let filename = "input.txt".to_string();

    let part_1_answer: u32 = day_01::solutions::distance(&filename);

    println!("Total distance is {part_1_answer}");

    let part_2_answer: u32 = day_01::solutions::similarity_score(&filename);

    println!("Similarity score is {part_2_answer}");
}
