pub mod cubes {
    use std::{cmp::max, error::Error, fs};

    struct GameResult {
        pub red: u32,
        pub green: u32,
        pub blue: u32,
    }

    impl GameResult {
        fn new(red: u32, green: u32, blue: u32) -> Self {
            Self { red, green, blue }
        }

        fn update_extreme_result(&mut self, red: u32, green: u32, blue: u32) {
            self.red = max(self.red, red);
            self.green = max(self.green, green);
            self.blue = max(self.blue, blue);
        }

        fn is_more_than(&self, other: &GameResult) -> bool {
            self.red >= other.red && self.green >= other.green && self.blue >= other.blue
        }

        fn power(&self) -> u32 {
            self.red * self.green * self.blue
        }
    }

    pub fn part_1(file_name: &String) -> Result<u32, Box<dyn Error>> {
        let contents = fs::read_to_string(file_name)?;

        let max_result = GameResult::new(12, 13, 14);

        let mut total: u32 = 0;

        for line in contents.lines() {
            let game_vec = &line.split(":").collect::<Vec<&str>>();
            let game_index: u32;
            let extreme_result: GameResult;

            if let Some(name) = game_vec.get(0) {
                if let Some(idx) = get_game_index(name) {
                    game_index = idx;
                } else {
                    return Err("Problem getting the game index!".into());
                }
            } else {
                return Err("Problem getting the game name!".into());
            }

            if let Some(results) = game_vec.get(1) {
                if let Some(result) = get_game_results(results) {
                    extreme_result = result;
                } else {
                    return Err("Problem getting the extreme results!".into());
                }
            } else {
                return Err("Problem getting the game results!".into());
            }
            if max_result.is_more_than(&extreme_result) {
                total += game_index;
            }
        }

        Ok(total)
    }

    pub fn part_2(file_name: &String) -> Result<u32, Box<dyn Error>> {
        let contents = fs::read_to_string(file_name)?;

        let mut total_power: u32 = 0;

        for line in contents.lines() {
            let game_vec = &line.split(":").collect::<Vec<&str>>();
            let extreme_result: GameResult;

            if let Some(results) = game_vec.get(1) {
                if let Some(result) = get_game_results(results) {
                    extreme_result = result;
                } else {
                    return Err("Problem getting the extreme results!".into());
                }
            } else {
                return Err("Problem getting the game results!".into());
            }
            total_power += extreme_result.power();
        }

        Ok(total_power)
    }

    fn get_game_index(game_name: &str) -> Option<u32> {
        let index: u32 = game_name
            .split(" ")
            .collect::<Vec<&str>>()
            .get(1)?
            .parse()
            .unwrap();

        Some(index)
    }

    fn get_game_results(results: &str) -> Option<GameResult> {
        let mut extreme_result: GameResult = GameResult::new(0, 0, 0);

        for result in results.split(";") {
            let (red, green, blue) = get_three_numbers(result);
            extreme_result.update_extreme_result(red, green, blue);
        }

        Some(extreme_result)
    }

    fn get_three_numbers(result: &str) -> (u32, u32, u32) {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for item in result.split(",").collect::<Vec<&str>>() {
            let item_vec = item.trim().split(" ").collect::<Vec<&str>>();
            let num = *item_vec.get(0).unwrap();
            let num: u32 = num.parse().unwrap();
            let color = *item_vec.get(1).unwrap();

            match color {
                "red" => red = max(red, num),
                "green" => green = max(green, num),
                "blue" => blue = max(blue, num),
                _ => panic!("Got invalid input!"),
            }
        }
        (red, green, blue)
    }
}
