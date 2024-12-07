#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

pub mod solutions {
    use std::{collections::HashSet, fs::read_to_string};

    use crate::Direction;

    pub fn num_visited_sites(filename: &String) -> u32 {
        //
        let content = read_to_string(filename).unwrap();

        let grid = content
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let (num_rows, num_cols) = (grid.len(), grid[0].len());
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        let (pos, dirn) = get_starting_state(&grid);
        let (mut row, mut col) = pos.unwrap();
        let mut dirn = dirn.unwrap();

        visited.insert((row, col));

        loop {
            let (delta_row, delta_col) = dirn.get_delta();
            let next_row = row as i32 + delta_row;
            let next_col = col as i32 + delta_col;

            if is_out_of_bounds((num_rows, num_cols), next_row, next_col) {
                break;
            }

            if grid[next_row as usize][next_col as usize] == '#' {
                dirn = dirn.turn_right();
            } else {
                row = next_row as usize;
                col = next_col as usize;
                visited.insert((row, col));
            }
        }

        visited.len() as u32
    }

    fn get_starting_state(grid: &Vec<Vec<char>>) -> (Option<(usize, usize)>, Option<Direction>) {
        let mut pos = None;
        let mut dirn = None;
        for (row_idx, line) in grid.iter().enumerate() {
            for (col_idx, ch) in line.iter().enumerate() {
                match ch {
                    '^' => {
                        pos = Some((row_idx, col_idx));
                        dirn = Some(Direction::Up);
                    }
                    'V' => {
                        pos = Some((row_idx, col_idx));
                        dirn = Some(Direction::Up);
                    }
                    '>' => {
                        pos = Some((row_idx, col_idx));
                        dirn = Some(Direction::Right);
                    }
                    '<' => {
                        pos = Some((row_idx, col_idx));
                        dirn = Some(Direction::Left);
                    }
                    _ => {}
                }
            }
        }
        (pos, dirn)
    }

    fn is_out_of_bounds(size: (usize, usize), next_row: i32, next_col: i32) -> bool {
        if next_row < 0 || next_col < 0 || next_row >= size.0 as i32 || next_col >= size.1 as i32 {
            return true;
        } else {
            return false;
        }
    }
}
