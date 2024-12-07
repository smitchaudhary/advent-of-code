use day_06::solutions::{count_loop_positions, num_visited_sites};

fn main() {
    let filename = "input.txt".to_string();
    let num_visits = num_visited_sites(&filename);
    println!("Number of distinct positions visited: {num_visits}");

    let num_loops = count_loop_positions(&filename);
    println!("Number of possible loops: {}", num_loops);
}
