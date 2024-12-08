pub mod solutions {
    use std::fs::read_to_string;

    pub fn count_calibration_value(filname: &String) -> i64 {
        let content = read_to_string(filname).unwrap();

        let mut calibration_value = 0;

        for line in content.lines() {
            let mut split = line.split(":");
            let lhs = split.next().unwrap().parse::<i64>().unwrap();
            let rhs = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            if count_valid(lhs, rhs) > 0 {
                calibration_value += lhs as i64;
            }
        }

        calibration_value
    }

    pub fn count_calibration_value_concatenated(filname: &String) -> i64 {
        // This is actually not a valid solution.
        // It is because it does not work as the current implementation does
        // the concatenation operation first and then follows the left to right
        // order of operations. But the question says that everything should follow
        // left to right.
        //
        // As an example,
        //
        // 1 + 2 ||3 * 4
        // should become (1+2) || 3 * 4
        // which is 33*4 = 132.
        //
        // But the current implementation will do
        // 1 + (23) * 4
        // 24*4 = 96
        let content = read_to_string(filname).unwrap();

        let mut calibration_value = 0;

        for line in content.lines() {
            let mut split = line.split(":");
            let lhs = split.next().unwrap().parse::<i64>().unwrap();
            let rhs = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            if count_valid_with_concatenated(lhs, rhs) > 0 {
                println!("{lhs}");
                calibration_value += lhs as i64;
            }
        }

        calibration_value
    }

    fn count_valid(lhs: i64, rhs: Vec<i64>) -> i64 {
        if lhs < 0 {
            return 0;
        } else if rhs.len() == 1 {
            if lhs == rhs[0] {
                return 1;
            } else {
                return 0;
            }
        } else {
            if lhs % rhs.last().unwrap() == 0 {
                return count_valid(lhs / rhs.last().unwrap(), rhs[..rhs.len() - 1].to_vec())
                    + count_valid(lhs - rhs.last().unwrap(), rhs[..rhs.len() - 1].to_vec());
            } else {
                return count_valid(lhs - rhs.last().unwrap(), rhs[..rhs.len() - 1].to_vec());
            }
        }
    }

    fn count_valid_with_concatenated(lhs: i64, rhs: Vec<i64>) -> i64 {
        if lhs < 0 {
            return 0;
        } else if rhs.len() == 1 {
            if lhs == rhs[0] {
                return 1;
            } else {
                return 0;
            }
        } else {
            let concatenated_rhs = concatenate_last_two(&rhs);
            if lhs % rhs.last().unwrap() == 0 {
                return count_valid_with_concatenated(
                    lhs / rhs.last().unwrap(),
                    rhs[..rhs.len() - 1].to_vec(),
                ) + count_valid_with_concatenated(
                    lhs - rhs.last().unwrap(),
                    rhs[..rhs.len() - 1].to_vec(),
                ) + count_valid_with_concatenated(lhs, concatenated_rhs);
            } else {
                return count_valid_with_concatenated(
                    lhs - rhs.last().unwrap(),
                    rhs[..rhs.len() - 1].to_vec(),
                ) + count_valid_with_concatenated(lhs, concatenated_rhs);
            }
        }
    }

    fn concatenate_last_two(rhs: &Vec<i64>) -> Vec<i64> {
        let num_digits = rhs.last().unwrap().to_string().len() as u32;
        let concatenated_num = rhs[rhs.len() - 2] * 10i64.pow(num_digits) + rhs.last().unwrap();
        let mut concatenated_rhs = rhs[..rhs.len() - 2].to_vec();
        concatenated_rhs.push(concatenated_num);
        concatenated_rhs
    }
}
