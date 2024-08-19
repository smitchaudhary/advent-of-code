pub mod navigation {
    use std::{collections::HashMap, error::Error, fs};

    pub fn run(file_name: &String) -> Result<u64, Box<dyn Error>> {
        let content = fs::read_to_string(file_name)?;

        let directions = content
            .lines()
            .nth(0)
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        let mut mapping: HashMap<&str, (&str, &str)> = HashMap::new();

        for line in content.lines().skip(2) {
            let current = line.split("=").nth(0).unwrap().trim();
            let trimmed_destinations = line
                .split("=")
                .nth(1)
                .unwrap()
                .trim()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap();

            let left = trimmed_destinations.split(",").nth(0).unwrap().trim();
            let right = trimmed_destinations.split(",").nth(1).unwrap().trim();

            mapping.insert(current, (left, right));
        }

        let mut num_steps = 0;

        let mut current_location = "AAA";

        while current_location != "ZZZ" {
            if directions[num_steps % directions.len()] == 'L' {
                current_location = mapping.get(current_location).unwrap().0;
            } else {
                current_location = mapping.get(current_location).unwrap().1;
            }
            num_steps += 1;
        }

        Ok(num_steps as u64)
    }

    pub fn simultaneous_run(file_name: &String) -> Result<u64, Box<dyn Error>> {
        let content = fs::read_to_string(file_name)?;

        let directions = content
            .lines()
            .nth(0)
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        let mut mapping: HashMap<&str, (&str, &str)> = HashMap::new();
        let mut current_locations: Vec<&str> = Vec::new();

        for line in content.lines().skip(2) {
            let current = line.split("=").nth(0).unwrap().trim();
            if current.chars().nth(2).unwrap() == 'A' {
                current_locations.push(current);
            }
            let trimmed_destinations = line
                .split("=")
                .nth(1)
                .unwrap()
                .trim()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap();

            let left = trimmed_destinations.split(",").nth(0).unwrap().trim();
            let right = trimmed_destinations.split(",").nth(1).unwrap().trim();

            mapping.insert(current, (left, right));
        }

        let mut num_steps = 0;

        let mut freq = vec![0; current_locations.len()];

        while freq.contains(&0) {
            for (idx, location) in current_locations.iter().enumerate() {
                if location.ends_with('Z') && freq[idx] == 0 {
                    freq[idx] = num_steps;
                }
            }
            if directions[num_steps % directions.len()] == 'L' {
                for location in current_locations.iter_mut() {
                    *location = mapping.get(location).unwrap().0;
                }
            } else {
                for location in current_locations.iter_mut() {
                    *location = mapping.get(location).unwrap().1;
                }
            }
            num_steps += 1;
        }

        Ok(get_lcm(freq))
    }

    fn get_lcm(freq: Vec<usize>) -> u64 {
        let a = freq[0] as u64;
        let b = freq[1] as u64;
        let mut lcm = lcm_fn(a, b);
        for i in 2..freq.len() {
            lcm = lcm_fn(lcm, freq[i] as u64);
        }
        lcm
    }

    fn lcm_fn(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }

    fn gcd(a: u64, b: u64) -> u64 {
        let (mut num, mut den) = match a > b {
            true => (a, b),
            false => (b, a),
        };
        let mut remainder = num % den;
        while remainder != 0 {
            num = den;
            den = remainder;
            remainder = num % den;
        }
        den
    }
}
