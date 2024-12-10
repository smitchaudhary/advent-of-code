use day_09::solutions::{get_checksum, get_checksum_block_moves};

fn main() {
    let filename = "input.txt".to_string();

    let file_checksum = get_checksum(&filename);

    println!("The file checksum is {file_checksum}");

    let file_checksum_with_blocks = get_checksum_block_moves(&filename);

    println!("The file checksum with moving blocks is {file_checksum_with_blocks}");
}
