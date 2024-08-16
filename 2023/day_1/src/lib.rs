pub mod part_1 {
    use std::{error::Error, fs};

    pub fn run(file_name: &String) -> Result<u32, Box<dyn Error>> {
        let contents = fs::read_to_string(file_name)?;

        let mut total: u32 = 0;

        for line in contents.lines() {
            'char_loop: for char in line.chars() {
                if char.is_numeric() {
                    if let Some(tens_digit) = char.to_digit(10) {
                        total += 10 * tens_digit;
                        break 'char_loop;
                    }
                }
            }
            'reverse_char_loop: for char in line.chars().rev() {
                if char.is_numeric() {
                    if let Some(ones_digit) = char.to_digit(10) {
                        total += ones_digit;
                        break 'reverse_char_loop;
                    }
                }
            }
        }

        Ok(total)
    }
}

pub mod part_2 {
    use std::{cmp::min, collections::HashMap, error::Error, fs};

    pub fn run(file_name: &String) -> Result<u32, Box<dyn Error>> {
        let contents = fs::read_to_string(file_name)?;

        let mut total: u32 = 0;

        for line in contents.lines() {
            total += get_calibration_from_line(line);
        }

        Ok(total)
    }

    fn create_hashmap(reversed: bool) -> HashMap<u32, String> {
        let mut num_to_spell: HashMap<u32, String> = HashMap::new();

        num_to_spell.insert(1, "one".to_string());
        num_to_spell.insert(2, "two".to_string());
        num_to_spell.insert(3, "three".to_string());
        num_to_spell.insert(4, "four".to_string());
        num_to_spell.insert(5, "five".to_string());
        num_to_spell.insert(6, "six".to_string());
        num_to_spell.insert(7, "seven".to_string());
        num_to_spell.insert(8, "eight".to_string());
        num_to_spell.insert(9, "nine".to_string());

        if reversed {
            for (_, value) in &mut num_to_spell {
                *value = reverse_string(&value);
            }
        }

        num_to_spell
    }

    fn get_calibration_from_line(line: &str) -> u32 {
        let tens_value: u32 = get_extreme_digit(line, &create_hashmap(false));
        let ones_value: u32 = get_extreme_digit(&reverse_string(line), &create_hashmap(true));
        10 * tens_value + ones_value
    }

    fn get_extreme_digit(line: &str, hmap: &HashMap<u32, String>) -> u32 {
        let mut index: u32 = line.len() as u32;
        let mut value: u32 = 0;
        for (num, spell) in hmap {
            let id1 = if let Some(idx) = line.find(spell) {
                idx as u32
            } else {
                line.len() as u32
            };

            let id2 = if let Some(idx) = line.find(&num.to_string()) {
                idx as u32
            } else {
                line.len() as u32
            };

            if min(id1, id2) < index {
                index = min(id1, id2);
                value = *num;
            }
        }
        value
    }

    fn reverse_string(input_str: &str) -> String {
        input_str.chars().rev().collect()
    }
}

pub mod part_3 {
    use std::{collections::HashMap, error::Error, fs};

    pub fn run(file_name: &String) -> Result<u32, Box<dyn Error>> {
        let mut contents = fs::read_to_string(file_name)?;

        let mut total: u32 = 0;

        let hmap = create_hashmap();

        for (k, v) in hmap {
            contents = contents.replace(v, k);
        }

        for line in contents.lines() {
            'char_loop: for char in line.chars() {
                if char.is_numeric() {
                    if let Some(tens_digit) = char.to_digit(10) {
                        total += 10 * tens_digit;
                        break 'char_loop;
                    }
                }
            }
            'reverse_char_loop: for char in line.chars().rev() {
                if char.is_numeric() {
                    if let Some(ones_digit) = char.to_digit(10) {
                        total += ones_digit;
                        break 'reverse_char_loop;
                    }
                }
            }
        }

        Ok(total)
    }

    fn create_hashmap() -> HashMap<&'static str, &'static str> {
        let mut num_to_spell: HashMap<&str, &str> = HashMap::new();

        num_to_spell.insert("1", "one");
        num_to_spell.insert("2", "two");
        num_to_spell.insert("3", "three");
        num_to_spell.insert("4", "four");
        num_to_spell.insert("5", "five");
        num_to_spell.insert("6", "six");
        num_to_spell.insert("7", "seven");
        num_to_spell.insert("8", "eight");
        num_to_spell.insert("9", "nine");

        num_to_spell
    }
}
