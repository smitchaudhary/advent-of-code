use day_07::solutions::count_calibration_value;

fn main() {
    let filename = "input.txt".to_string();

    let calibration_value = count_calibration_value(&filename);

    println!("The number of ways with valid calculations is {calibration_value}");
}
