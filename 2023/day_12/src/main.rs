fn main() {
    let file_name = "sample.txt".to_string();

    let part_1_answer = day_12::springs::part_1(&file_name);

    println!("The total possible combinations: {}", part_1_answer);

    println!(
        "This seems to be ok logic wise! But no idea how to cache stuff in \
        Rust yet to avoid repeating calculations on same options. Or use DP or such concepts."
    );
    let part_2_answer = day_12::springs::part_2(&file_name);

    println!("The total expanded combinations: {}", part_2_answer);
}
