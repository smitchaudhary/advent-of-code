use day_03::solution::{mul_total, mul_total_do_dont};

fn main() {
    let filename = "input.txt".to_string();

    let total = mul_total(&filename);

    println!("The total of the uncorrupted mul is {total}");

    let do_dont_total = mul_total_do_dont(&filename);

    println!("The total with dos and don'ts is {do_dont_total}");
}
