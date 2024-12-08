use day_07::solutions::{count_calibration_value, count_calibration_value_concatenated};

fn main() {
    let filename = "sample.txt".to_string();

    let calibration_value = count_calibration_value(&filename);

    println!("The calibration value is {calibration_value}");

    let concatenated_calibration_value = count_calibration_value_concatenated(&filename);

    println!("The calibration value with concatenation is {concatenated_calibration_value}");
}
