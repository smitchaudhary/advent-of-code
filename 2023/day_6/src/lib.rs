pub mod race {
    use std::{error::Error, fs};

    pub fn part_1(file_name: &String) -> Result<i32, Box<dyn Error>> {
        let contents = fs::read_to_string(file_name)?;

        let mut times: Vec<f64> = Vec::new();
        let mut distances: Vec<f64> = Vec::new();

        for num in contents
            .lines()
            .nth(0)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
        {
            let number = num.parse::<f64>().unwrap();
            times.push(number);
        }

        for dist in contents
            .lines()
            .nth(1)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
        {
            let distance = dist.parse::<f64>().unwrap();
            distances.push(distance);
        }

        let num_races = times.len();

        let mut num_ways = 1;

        for index in 0..num_races {
            num_ways *=
                num_winning_ways(*times.get(index).unwrap(), *distances.get(index).unwrap());
        }

        Ok(num_ways)
    }

    pub fn part_2(file_name: &String) -> Result<i32, Box<dyn Error>> {
        let contents = fs::read_to_string(file_name)?;

        let time = contents
            .lines()
            .nth(0)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect::<String>()
            .parse::<f64>()
            .unwrap();

        let dist = contents
            .lines()
            .nth(1)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect::<String>()
            .parse::<f64>()
            .unwrap();
        Ok(num_winning_ways(time, dist))
    }

    fn num_winning_ways(time: f64, dist: f64) -> i32 {
        let mut count = 0;

        let disc = (time * time - 4.0 * dist).sqrt();

        if (time / 2.0).fract() == 0.0 {
            if (disc / 2.0).fract() == 0.0 {
                count += 2 * ((disc / 2.0).floor() as i32) - 1;
            } else {
                count += 2 * ((disc / 2.0).floor() as i32) + 1;
            }
        } else {
            count += 2 * ((disc / 2.0 - 0.5).floor() as i32);
            if (disc / 2.0 - 0.5).fract() != 0.0 {
                count += 2;
            }
        }
        count
    }
}
