use day_13::solutions::{get_tokens, get_tokens_converted};

fn main() {
    let filename = "input.txt".to_string();

    let tokens = get_tokens(&filename);

    println!("Total {tokens} spent");

    let corrected_tokens = get_tokens_converted(&filename);

    println!("Total tokens in second part: {corrected_tokens}");
}
