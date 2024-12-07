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
}
