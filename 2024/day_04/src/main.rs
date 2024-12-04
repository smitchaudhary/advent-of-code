use day_04::solutions::{part_1, part_2};

fn main() {
    let filename = "input.txt".to_string();

    let xmas_count = part_1(&filename);
    println!("Part 1 - The total XMAS count is: {xmas_count}");

    let xmas_pattern_count = part_2(&filename);
    println!("Part 2 - The total X-MAS pattern count is: {xmas_pattern_count}");
}
