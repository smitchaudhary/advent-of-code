pub mod solutions {
    use std::fs::read_to_string;

    pub fn middle_of_correct_update(filename: &String) -> u32 {
        let content = read_to_string(filename).unwrap();
        let mut content_split = content.split("\n\n");

        let mut middle_num: u32 = 0;

        let rules: Vec<(u32, u32)> = content_split
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let parts = line.split("|").collect::<Vec<&str>>();
                (
                    parts[0].parse::<u32>().unwrap(),
                    parts[1].parse::<u32>().unwrap(),
                )
            })
            .collect();

        'line_loop: for line in content_split.next().unwrap().lines() {
            let update = line
                .split(",")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            for (a, b) in &rules {
                match (
                    update.iter().position(|&x| x == *a),
                    update.iter().position(|&x| x == *b),
                ) {
                    (Some(a_idx), Some(b_idx)) => {
                        if a_idx > b_idx {
                            continue 'line_loop;
                        } else {
                        }
                    }
                    _ => {}
                }
            }
            middle_num += update[(update.len() - 1) / 2];
        }

        middle_num
    }

    pub fn middle_of_incorrect_update(filename: &String) -> u32 {
        let content = read_to_string(filename).unwrap();
        let mut content_split = content.split("\n\n");

        let mut middle_num: u32 = 0;

        let rules: Vec<(u32, u32)> = content_split
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let parts = line.split("|").collect::<Vec<&str>>();
                (
                    parts[0].parse::<u32>().unwrap(),
                    parts[1].parse::<u32>().unwrap(),
                )
            })
            .collect();

        for line in content_split.next().unwrap().lines() {
            let mut update = line
                .split(",")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let mut need_resort: bool = true;
            while need_resort {
                need_resort = false;
                for (a, b) in &rules {
                    match (
                        update.iter().position(|&x| x == *a),
                        update.iter().position(|&x| x == *b),
                    ) {
                        (Some(a_idx), Some(b_idx)) => {
                            if a_idx > b_idx {
                                update.swap(a_idx, b_idx);
                                need_resort = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
            middle_num += update[(update.len() - 1) / 2];
        }

        middle_num
    }
}
