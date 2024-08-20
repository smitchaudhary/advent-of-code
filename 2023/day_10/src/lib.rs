pub mod pipes {
    use std::fs;

    pub fn length(file_name: &String) -> u32 {
        let content = fs::read_to_string(file_name).unwrap();

        let mut diagram: Vec<Vec<char>> = Vec::new();

        let mut starting_location: (usize, usize) = (0, 0);

        for (row, line) in content.lines().enumerate() {
            if line.contains('S') {
                starting_location = (row, line.find('S').unwrap())
            }
            diagram.push(line.chars().collect::<Vec<char>>());
        }

        max_dist(&diagram, starting_location).unwrap()
    }

    pub fn interior_area(file_name: &String) -> u32 {
        let content = fs::read_to_string(file_name).unwrap();

        let mut diagram: Vec<Vec<char>> = Vec::new();

        let mut starting_location: (usize, usize) = (0, 0);

        for (row, line) in content.lines().enumerate() {
            if line.contains('S') {
                starting_location = (row, line.find('S').unwrap())
            }
            diagram.push(line.chars().collect::<Vec<char>>());
        }

        get_loop_area(&diagram, starting_location).unwrap()
            - max_dist(&diagram, starting_location).unwrap()
            + 1
    }

    fn max_dist(diagram: &Vec<Vec<char>>, starting_location: (usize, usize)) -> Option<u32> {
        let dirn: Direction = starting_direction(starting_location, &diagram)?;
        let (mut dirn, mut location) = take_step(dirn, starting_location, &diagram)?;
        let mut loop_length = 2;

        while location != starting_location {
            (dirn, location) = take_step(dirn, location, &diagram)?;
            loop_length += 1;
        }

        Some(loop_length / 2)
    }

    fn get_loop_area(diagram: &Vec<Vec<char>>, starting_location: (usize, usize)) -> Option<u32> {
        let dirn: Direction = starting_direction(starting_location, &diagram)?;
        let (mut dirn, mut location) = take_step(dirn, starting_location, &diagram)?;
        let mut area =
            (starting_location.0 * location.1) as i32 - (starting_location.1 * location.0) as i32;
        while location != starting_location {
            let (x1, y1) = (location.0 as i32, location.1 as i32);
            (dirn, location) = take_step(dirn, location, &diagram)?;
            let (x2, y2) = (location.0 as i32, location.1 as i32);
            area += x1 * y2 - x2 * y1;
        }
        Some((area.abs() / 2) as u32)
    }

    fn starting_direction(
        starting_location: (usize, usize),
        diagram: &Vec<Vec<char>>,
    ) -> Option<Direction> {
        let (row, col) = starting_location;
        let height = diagram.len();
        let width = diagram[0].len();

        if row > 0 && ['|', '7', 'F'].contains(&diagram[row - 1][col]) {
            return Some(Direction::Up);
        }

        if row + 1 < height && ['|', 'J', 'L'].contains(&diagram[row + 1][col]) {
            return Some(Direction::Down);
        }

        if col > 0 && ['-', 'L', 'F'].contains(&diagram[row][col - 1]) {
            return Some(Direction::Left);
        }

        if col + 1 < width && ['-', 'J', '7'].contains(&diagram[row][col + 1]) {
            return Some(Direction::Right);
        }

        None
    }

    fn take_step(
        dirn: Direction,
        location: (usize, usize),
        diagram: &Vec<Vec<char>>,
    ) -> Option<(Direction, (usize, usize))> {
        let (row, col) = location;
        let (new_row, new_col) = match dirn {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        };

        if diagram[new_row][new_col] == 'S' {
            return Some((dirn, (new_row, new_col)));
        }
        match dirn {
            Direction::Up => match diagram[new_row][new_col] {
                '|' => return Some((Direction::Up, (new_row, new_col))),
                '7' => return Some((Direction::Left, (new_row, new_col))),
                'F' => return Some((Direction::Right, (new_row, new_col))),
                _ => return None,
            },
            Direction::Down => match diagram[new_row][new_col] {
                '|' => return Some((Direction::Down, (new_row, new_col))),
                'L' => return Some((Direction::Right, (new_row, new_col))),
                'J' => return Some((Direction::Left, (new_row, new_col))),
                _ => return None,
            },
            Direction::Left => match diagram[new_row][new_col] {
                '-' => return Some((Direction::Left, (new_row, new_col))),
                'L' => return Some((Direction::Up, (new_row, new_col))),
                'F' => return Some((Direction::Down, (new_row, new_col))),
                _ => return None,
            },
            Direction::Right => match diagram[new_row][new_col] {
                '-' => return Some((Direction::Right, (new_row, new_col))),
                '7' => return Some((Direction::Down, (new_row, new_col))),
                'J' => return Some((Direction::Up, (new_row, new_col))),
                _ => return None,
            },
        }
    }

    enum Direction {
        Up,
        Down,
        Right,
        Left,
    }
}
