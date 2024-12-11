use day_11::solutions::get_num_stones;

fn main() {
    let filename = "input.txt".to_string();

    let num_blinks = [25, 75];
    // I need to learn either about DP or about how to write more performant code
    // if I can't make it run in reasonably time even in Rust (preferably both).

    for num in num_blinks {
        let num_stones = get_num_stones(&filename, num);

        println!("After {num} blinks, we have {num_stones} stones");
    }
}
