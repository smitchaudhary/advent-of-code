use day_14::solutions::get_safety_factor;

fn main() {
    let filename = "input.txt".to_string();

    let size = if filename == "sample.txt" {
        vec![11, 7]
    } else {
        vec![101, 103]
    };

    let safety_factor = get_safety_factor(&filename, size);

    println!("The safety factor is: {safety_factor}");
}
