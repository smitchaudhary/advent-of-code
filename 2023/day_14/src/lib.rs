pub mod part_1 {
    use std::fs;

    pub fn run(file_name: &String) -> i32 {
        let rocks = fs::read_to_string(file_name)
            .unwrap()
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut total_load = 0;

        for i in 0..rocks[0].len() {
            let mut next_pos = rocks.len() as i32;
            for j in 0..rocks.len() {
                if rocks[j][i] == 'O' {
                    total_load += next_pos;
                    next_pos -= 1;
                } else if rocks[j][i] == '#' {
                    next_pos = (rocks.len() - j) as i32 - 1;
                }
            }
        }

        total_load
    }
}
