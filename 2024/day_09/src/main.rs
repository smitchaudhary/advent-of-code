use day_09::solutions::get_checksum;

fn main() {
    let filename = "input.txt".to_string();

    let file_checksum = get_checksum(&filename);

    println!("The file checksum is {file_checksum}");
}
