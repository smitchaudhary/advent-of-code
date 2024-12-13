use day_13::solutions::get_tokens;

fn main() {
    let filename = "input.txt".to_string();

    let tokens = get_tokens(&filename);

    println!("Total {tokens} spent");
}
