use day_10::solutions::{get_total_score, get_total_score_repeated};

fn main() {
    let filename = "input.txt".to_string();

    let total_trailhead_score = get_total_score(&filename);

    println!("Total trailhead score is {total_trailhead_score}");

    let total_trailhead_score_repeated = get_total_score_repeated(&filename);

    println!("Total trailhead score with repeats is {total_trailhead_score_repeated}");
}
