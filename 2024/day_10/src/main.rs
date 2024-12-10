use day_10::solutions::get_total_score;

fn main() {
    let filename = "input.txt".to_string();

    let total_trailhead_score = get_total_score(&filename);

    println!("Total trailhead score is {total_trailhead_score}");
}
