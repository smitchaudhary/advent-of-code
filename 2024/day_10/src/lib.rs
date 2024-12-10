pub mod solutions {
    use std::{collections::HashSet, fs::read_to_string};

    pub fn get_total_score(filename: &String) -> u32 {
        let content = read_to_string(filename).unwrap();

        let grid = content
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let trailheads: Vec<(i32, i32)> = get_trailheads(&grid);

        let mut total_score = 0;

        for head in trailheads {
            let mut visited: HashSet<(i32, i32)> = HashSet::new();

            total_score += trailhead_score(&grid, head, &mut visited, true);
        }

        total_score
    }

    pub fn get_total_score_repeated(filename: &String) -> u32 {
        let content = read_to_string(filename).unwrap();

        let grid = content
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let trailheads: Vec<(i32, i32)> = get_trailheads(&grid);

        let mut total_score = 0;

        for head in trailheads {
            let mut visited: HashSet<(i32, i32)> = HashSet::new();

            total_score += trailhead_score(&grid, head, &mut visited, false);
        }

        total_score
    }

    fn get_trailheads(grid: &Vec<Vec<u32>>) -> Vec<(i32, i32)> {
        let mut trailheads: Vec<(i32, i32)> = Vec::new();

        for row_idx in 0..grid.len() {
            for col_idx in 0..grid[0].len() {
                if grid[row_idx][col_idx] == 0 {
                    trailheads.push((row_idx as i32, col_idx as i32));
                }
            }
        }

        trailheads
    }

    fn trailhead_score(
        grid: &Vec<Vec<u32>>,
        pos: (i32, i32),
        visited: &mut HashSet<(i32, i32)>,
        skip_visited: bool,
    ) -> u32 {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= grid.len() as i32 || pos.1 >= grid[0].len() as i32 {
            return 0;
        }

        if visited.contains(&pos) && skip_visited {
            return 0;
        }

        visited.insert(pos);

        if grid[pos.0 as usize][pos.1 as usize] == 9 {
            return 1;
        }

        let mut total_score: u32 = 0;
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for dirn in directions {
            let new_pos = (pos.0 + dirn.0, pos.1 + dirn.1);
            if new_pos.0 >= 0
                && new_pos.1 >= 0
                && new_pos.0 < grid.len() as i32
                && new_pos.1 < grid[0].len() as i32
            {
                if grid[new_pos.0 as usize][new_pos.1 as usize]
                    == grid[pos.0 as usize][pos.1 as usize] + 1
                {
                    total_score += trailhead_score(grid, new_pos, visited, skip_visited);
                }
            }
        }

        total_score
    }
}
