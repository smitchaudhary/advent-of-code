pub mod solutions {
    use std::fs::read_to_string;

    pub fn get_num_stones(filename: &String, num_blinks: u32) -> u32 {
        let content = read_to_string(filename).unwrap();

        let mut arrangement = content
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        for i in 0..num_blinks {
            println!("After {} blinks", i + 1);
            blink(&mut arrangement);
        }
        arrangement.len() as u32
    }

    fn blink(arrangement: &mut Vec<u64>) {
        let mut idx: usize = 0;
        while idx < arrangement.len() {
            if arrangement[idx] == 0 {
                arrangement[idx] = 1;
            } else if arrangement[idx].to_string().len() % 2 == 0 {
                let stone_number = arrangement[idx].to_string();
                let first_half = stone_number[..stone_number.len() / 2]
                    .parse::<u64>()
                    .unwrap();
                let second_half = stone_number[stone_number.len() / 2..]
                    .parse::<u64>()
                    .unwrap();
                arrangement[idx] = first_half;
                arrangement.insert(idx + 1, second_half);
                idx += 1;
            } else {
                arrangement[idx] *= 2024;
            }
            idx += 1;
        }
    }
}
