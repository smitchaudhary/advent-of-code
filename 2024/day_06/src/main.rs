use day_06::solutions::num_visited_sites;

fn main() {
    let filename = "sample.txt".to_string();
    let num_visits = num_visited_sites(&filename);
    println!("Number of distinct positions visited: {num_visits}");

    // let result = num_visited_sites(&filename);
    // println!("Number of distinct positions visited: {}", result);
}
