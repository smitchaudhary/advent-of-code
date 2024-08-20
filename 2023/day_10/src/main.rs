fn main() {
    let file_name = "input.txt".to_string();

    let part_1_answer = day_10::pipes::length(&file_name);

    println!("Distance to farthest point is: {}", { part_1_answer });

    let part_2_answer = day_10::pipes::interior_area(&file_name);

    println!("Total area enclosed is: {}", { part_2_answer });
}
