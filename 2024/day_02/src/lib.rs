pub mod solutions {
    use std::fs::read_to_string;

    pub fn count_safe_reports(filename: &str) -> u32 {
        let contents = read_to_string(filename).unwrap();

        let lines = contents.lines().map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        });

        let mut num_safe_reports: u32 = 0;

        for report in lines {
            if is_safe(&report) {
                num_safe_reports += 1;
            }
        }

        num_safe_reports
    }

    pub fn count_damped_safe_reports(filename: &str) -> u32 {
        let contents = read_to_string(filename).unwrap();

        let lines = contents.lines().map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        });

        let mut num_safe_reports: u32 = 0;

        for report in lines {
            if is_safe(&report) {
                num_safe_reports += 1;
            } else if is_safe_damped(&report) {
                num_safe_reports += 1;
            }
        }

        num_safe_reports
    }

    fn is_safe(report: &Vec<i32>) -> bool {
        if report[1] > report[0] {
            let mut diff = report.windows(2).map(|pair| pair[1] - pair[0]);
            diff.all(|diff| diff >= 1 && diff <= 3)
        } else {
            let mut diff = report.windows(2).map(|pair| pair[0] - pair[1]);
            diff.all(|diff| diff >= 1 && diff <= 3)
        }
    }

    fn is_safe_damped(report: &Vec<i32>) -> bool {
        for idx in 0..report.len() {
            if is_safe(
                &report
                    .into_iter()
                    .enumerate()
                    .filter(|(i, _)| *i != idx)
                    .map(|(_, x)| *x)
                    .collect(),
            ) {
                return true;
            }
        }
        false
    }
}
