pub mod solutions {
    use std::fs::read_to_string;

    pub fn get_tokens(filename: &String) -> i64 {
        let contents = read_to_string(filename).unwrap();

        let mut tokens = 0;

        for game in contents.split("\n\n") {
            let (button_a, button_b, target) = get_machine_stats(game);

            tokens += get_cost(button_a, button_b, target);
        }

        tokens
    }

    pub fn get_tokens_converted(filename: &String) -> i64 {
        let contents = read_to_string(filename).unwrap();

        let mut tokens = 0;

        for game in contents.split("\n\n") {
            let (button_a, button_b, target) = get_machine_stats(game);
            let target = (target.0 + 10000000000000, target.1 + 10000000000000);
            tokens += get_cost(button_a, button_b, target);
        }

        tokens
    }

    fn get_machine_stats(input: &str) -> ((i64, i64), (i64, i64), (i64, i64)) {
        let mut lines = input.lines();
        let button_a = lines
            .next()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|cap| cap.split("+").nth(1).unwrap().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let button_b = lines
            .next()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|cap| cap.split("+").nth(1).unwrap().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let prize = lines
            .next()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|cap| cap.split("=").nth(1).unwrap().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        (
            (button_a[0], button_a[1]),
            (button_b[0], button_b[1]),
            (prize[0], prize[1]),
        )
    }

    fn get_cost(button_a: (i64, i64), button_b: (i64, i64), prize: (i64, i64)) -> i64 {
        let det = button_a.0 * button_b.1 - button_a.1 * button_b.0;

        let a = (button_b.1 * prize.0 - button_b.0 * prize.1) / det;
        let b = (-button_a.1 * prize.0 + button_a.0 * prize.1) / det;

        if a * button_a.0 + b * button_b.0 == prize.0 && a * button_a.1 + b * button_b.1 == prize.1
        {
            return 3 * a + b;
        }
        0
    }
}
