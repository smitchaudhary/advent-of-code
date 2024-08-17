pub mod scratch_cards {
    use std::{collections::HashMap, error::Error, fs};

    pub fn part_1(file_name: &String) -> Result<u32, Box<dyn Error>> {
        let content = fs::read_to_string(file_name)?;

        let mut total = 0;

        for line in content.lines() {
            let output = u32::pow(2, points_in_card(line)) / 2;
            total += output;
        }

        Ok(total)
    }

    pub fn part_2(file_name: &String) -> Result<u32, Box<dyn Error>> {
        let content = fs::read_to_string(file_name)?;
        let lines = content.lines().collect::<Vec<&str>>();
        let mut total_scratch_cards = lines.len() as u32;

        let mut hmap: HashMap<usize, u32> = HashMap::new();

        for i in 0..lines.len() {
            hmap.insert(i, 1);
        }

        for i in 0..lines.len() {
            let card = *lines.get(i).unwrap();
            let num_matches = points_in_card(card);
            total_scratch_cards += hmap.get(&i).unwrap() * num_matches;

            for j in (i + 1)..(i + 1 + num_matches as usize) {
                hmap.insert(j, hmap[&j] + hmap.get(&i).unwrap());
            }
        }

        Ok(total_scratch_cards)
    }

    fn points_in_card(card: &str) -> u32 {
        let mut total_matches = 0;

        let card_values = card
            .split(":")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .split("|")
            .collect::<Vec<&str>>();

        let winning_nums = card_values
            .get(0)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        let my_nums = card_values
            .get(1)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        for item in &my_nums {
            for win_num in &winning_nums {
                if item == win_num {
                    total_matches += 1;
                }
            }
        }

        total_matches
    }
}

// For part 2:
// Go over the lines one by one.
// When at line k, find how many matches, let this number be mtc
// See how many copies of line k you had from hashmap, let this number be cp
// Add cp to the hashmap for the next mtc lines.
// Add mtc*cp to total scratch cards
