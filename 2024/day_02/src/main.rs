use day_02::solutions::{count_damped_safe_reports, count_safe_reports};

fn main() {
    let filename = "input.txt".to_string();

    let num_safe_reports = count_safe_reports(&filename);

    println!("The number of safe reports are {num_safe_reports}");

    let num_damped_safe_reports = count_damped_safe_reports(&filename);

    println!("THe number of damped safe reports: {num_damped_safe_reports}");
}
