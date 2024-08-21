pub mod springs {
    use std::fs;

    pub fn part_1(file_name: &String) -> u32 {
        let content = fs::read_to_string(file_name).unwrap();

        let mut total_num_ways = 0;

        for line in content.lines() {
            total_num_ways += get_num_ways(line).unwrap();
        }

        total_num_ways
    }

    pub fn part_2(file_name: &String) -> u64 {
        let content = fs::read_to_string(file_name).unwrap();

        let mut total_num_ways = 0;

        for line in content.lines() {
            total_num_ways += get_num_ways_part_2(line).unwrap() as u64;
        }

        total_num_ways
    }

    fn get_num_ways(line: &str) -> Option<u32> {
        let springs = line
            .split_whitespace()
            .nth(0)
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        let non_working_groups = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(",")
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        Some(count_ways(&springs, &non_working_groups, 0, false))
    }

    fn get_num_ways_part_2(line: &str) -> Option<u32> {
        let springs = line
            .split_whitespace()
            .nth(0)
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        let non_working_groups = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(",")
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut expanded_springs = springs.clone();
        let mut expanded_groups = non_working_groups.clone();

        for _ in 0..4 {
            expanded_springs.extend(['?'].iter());
            expanded_springs.extend(springs.iter());
            expanded_groups.extend(non_working_groups.iter());
        }

        Some(count_ways(&expanded_springs, &expanded_groups, 0, false))
    }

    fn count_ways(
        remaining_springs: &[char],
        remaining_groups: &[u32],
        current_non_working_size: u32,
        next_must_work: bool,
    ) -> u32 {
        if remaining_springs.is_empty() {
            if !remaining_groups.is_empty() {
                if remaining_groups.len() == 1 && remaining_groups[0] == current_non_working_size {
                    return 1;
                } else {
                    return 0;
                }
            } else {
                if current_non_working_size == 0 {
                    return 1;
                } else {
                    return 0;
                }
            }
        }

        if remaining_groups.is_empty() {
            if remaining_springs.contains(&'#') {
                return 0;
            } else {
                return 1;
            }
        }

        let next_spring = remaining_springs[0];

        if next_spring == '#' {
            if next_must_work {
                return 0;
            }

            let new_non_working_size = current_non_working_size + 1;

            if new_non_working_size > remaining_groups[0] {
                return 0;
            } else if new_non_working_size == remaining_groups[0] {
                return count_ways(&remaining_springs[1..], &remaining_groups[1..], 0, true);
            } else {
                return count_ways(
                    &remaining_springs[1..],
                    remaining_groups,
                    new_non_working_size,
                    false,
                );
            }
        } else if next_spring == '.' {
            if next_must_work {
                return count_ways(&remaining_springs[1..], remaining_groups, 0, false);
            }

            if current_non_working_size == 0 {
                return count_ways(
                    &remaining_springs[1..],
                    remaining_groups,
                    current_non_working_size,
                    false,
                );
            } else {
                if current_non_working_size != remaining_groups[0] {
                    return 0;
                } else {
                    count_ways(&remaining_springs[1..], &remaining_groups[1..], 0, false);
                }
            }
        } else if next_spring == '?' {
            let mut broken_replacement_springs = remaining_springs.to_vec();
            broken_replacement_springs[0] = '#';
            let broken_count = count_ways(
                &broken_replacement_springs,
                remaining_groups,
                current_non_working_size,
                next_must_work,
            );
            let mut working_replacement_springs = remaining_springs.to_vec();
            working_replacement_springs[0] = '.';
            let working_count = count_ways(
                &working_replacement_springs,
                remaining_groups,
                current_non_working_size,
                next_must_work,
            );

            return working_count + broken_count;
        }

        0
    }
}
