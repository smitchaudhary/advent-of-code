use day_08::solutions::{count_antinodes, count_resonant_antinodes};

fn main() {
    let filename = "input.txt".to_string();

    let num_antinodes = count_antinodes(&filename);

    println!("The number of antinodes is {num_antinodes}");

    let num_resonant_antinodes = count_resonant_antinodes(&filename);

    println!("The number of resonant antinodes is {num_resonant_antinodes}");
}
