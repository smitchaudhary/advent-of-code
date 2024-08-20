pub mod part_1 {
    use std::fs;

    pub fn run(file_name: &String) -> i64 {
        let report = fs::read_to_string(file_name).unwrap();

        let mut total_extrapolation = 0;

        for line in report.lines() {
            let mut history = Vec::new();
            for num in line.split_whitespace() {
                history.push(num.parse::<i64>().unwrap());
            }
            let exptrapolation = get_extrapolation(history);
            total_extrapolation += exptrapolation;
        }

        total_extrapolation
    }

    fn get_extrapolation(history: Vec<i64>) -> i64 {
        let mut cloned_history = history.clone();
        cloned_history.dedup();
        if cloned_history.len() == 1 {
            return *history.last().unwrap();
        } else {
            let mut diff_history = Vec::new();
            for i in 1..history.len() {
                diff_history.push(history[i] - history[i - 1]);
            }
            return history.last().unwrap() + get_extrapolation(diff_history);
        }
    }
}

pub mod part_2 {
    use std::fs;

    pub fn run(file_name: &String) -> i64 {
        let report = fs::read_to_string(file_name).unwrap();

        let mut total_extrapolation = 0;

        for line in report.lines() {
            let mut history = Vec::new();
            for num in line.split_whitespace() {
                history.push(num.parse::<i64>().unwrap());
            }
            let exptrapolation = get_back_extrapolation(history, 1);
            total_extrapolation += exptrapolation;
        }

        total_extrapolation
    }

    fn get_back_extrapolation(history: Vec<i64>, pre_factor: i64) -> i64 {
        let mut cloned_history = history.clone();
        cloned_history.dedup();
        if cloned_history.len() == 1 {
            return pre_factor * history.first().unwrap();
        } else {
            let mut diff_history = Vec::new();
            for i in 1..history.len() {
                diff_history.push(history[i] - history[i - 1]);
            }
            return pre_factor * history.first().unwrap()
                + get_back_extrapolation(diff_history, -1 * pre_factor);
        }
    }
}
