pub mod solutions {
    use std::{collections::HashSet, fs::read_to_string};

    pub fn get_gps_coords(filename: &String) -> u32 {
        let content = read_to_string(filename).unwrap();

        let mut map = content
            .split("\n\n")
            .nth(0)
            .unwrap()
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let instructions = content.split("\n\n").nth(1).unwrap().replace("\n", "");

        let mut bot_pos = initial_pos(&map);

        for dirn in instructions.chars() {
            make_move(&mut map, dirn, &mut bot_pos);
        }

        count_gps(&map)
    }

    pub fn get_widened_gps_coords(filename: &String) -> u32 {
        let content = read_to_string(filename).unwrap();

        let map = content
            .split("\n\n")
            .nth(0)
            .unwrap()
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut map = widen(map);

        let instructions = content.split("\n\n").nth(1).unwrap().replace("\n", "");

        let mut bot_pos = initial_pos(&map);

        for dirn in instructions.chars() {
            make_move_widened(&mut map, dirn, &mut bot_pos);
        }

        count_gps(&map)
    }

    fn make_move(map: &mut Vec<Vec<char>>, dirn: char, bot_pos: &mut (usize, usize)) {
        let dx: (i32, i32) = match dirn {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => panic!("Invalid character"),
        };
        let next_pos = (
            (bot_pos.0 as i32 + dx.0) as usize,
            (bot_pos.1 as i32 + dx.1) as usize,
        );
        match map[next_pos.0][next_pos.1] {
            '.' => {
                map[bot_pos.0][bot_pos.1] = '.';
                map[next_pos.0][next_pos.1] = '@';
                *bot_pos = next_pos;
            }
            'O' => {
                let mut next_search_pos = next_pos;

                while map[next_search_pos.0][next_search_pos.1] != '#' {
                    next_search_pos = (
                        (next_search_pos.0 as i32 + dx.0) as usize,
                        (next_search_pos.1 as i32 + dx.1) as usize,
                    );
                    if map[next_search_pos.0][next_search_pos.1] == '.' {
                        map[next_search_pos.0][next_search_pos.1] = 'O';
                        map[next_pos.0][next_pos.1] = '@';
                        map[bot_pos.0][bot_pos.1] = '.';
                        *bot_pos = next_pos;
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    fn make_move_widened(map: &mut Vec<Vec<char>>, dirn: char, bot_pos: &mut (usize, usize)) {
        let dx: (i32, i32) = match dirn {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => panic!("Invalid character"),
        };

        let next_pos = (
            (bot_pos.0 as i32 + dx.0) as usize,
            (bot_pos.1 as i32 + dx.1) as usize,
        );

        match map[next_pos.0][next_pos.1] {
            '.' => {
                map[bot_pos.0][bot_pos.1] = '.';
                map[next_pos.0][next_pos.1] = '@';
                *bot_pos = next_pos;
            }
            '[' | ']' => {
                let mut checked: HashSet<(usize, usize)> = HashSet::new();

                if !check_movable(map, *bot_pos, dx, &mut checked) {
                    return;
                }

                while !checked.is_empty() {
                    let mut to_remove = Vec::new();

                    for &pos in &checked {
                        let next_pos = (
                            (pos.0 as i32 + dx.0) as usize,
                            (pos.1 as i32 + dx.1) as usize,
                        );

                        if !checked.contains(&next_pos) {
                            if map[next_pos.0][next_pos.1] != '@' && map[pos.0][pos.1] != '@' {
                                map[next_pos.0][next_pos.1] = map[pos.0][pos.1];
                                map[pos.0][pos.1] = '.';
                            }
                            to_remove.push(pos);
                        }
                    }

                    for pos in to_remove {
                        checked.remove(&pos);
                    }
                }

                map[next_pos.0][next_pos.1] = '@';
                map[bot_pos.0][bot_pos.1] = '.';
                *bot_pos = next_pos;
            }
            _ => {}
        }
    }

    fn initial_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
        let row_idx = map.iter().position(|line| line.contains(&'@')).unwrap();

        let col_idx = map[row_idx].iter().position(|&ch| ch == '@').unwrap();

        (row_idx, col_idx)
    }

    fn count_gps(map: &Vec<Vec<char>>) -> u32 {
        let mut gps = 0;
        for (row_idx, line) in map.iter().enumerate() {
            for (col_idx, &ch) in line.iter().enumerate() {
                if ch == 'O' || ch == '[' {
                    gps += row_idx * 100 + col_idx;
                }
            }
        }
        gps as u32
    }

    fn check_movable(
        map: &Vec<Vec<char>>,
        pos: (usize, usize),
        dx: (i32, i32),
        seen: &mut HashSet<(usize, usize)>,
    ) -> bool {
        if !seen.insert(pos) {
            return true;
        }

        let next_pos = (
            (pos.0 as i32 + dx.0) as usize,
            (pos.1 as i32 + dx.1) as usize,
        );

        match map[next_pos.0][next_pos.1] {
            '#' => false,
            '[' => {
                check_movable(map, next_pos, dx, seen)
                    && check_movable(map, (next_pos.0, next_pos.1 + 1), dx, seen)
            }
            ']' => {
                check_movable(map, next_pos, dx, seen)
                    && check_movable(map, (next_pos.0, next_pos.1 - 1), dx, seen)
            }
            _ => true,
        }
    }

    fn widen(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut widened_map: Vec<Vec<char>> = Vec::new();
        for row in map.iter() {
            let mut new_row: Vec<char> = Vec::new();
            for &cell in row {
                match cell {
                    '#' => {
                        new_row.push('#');
                        new_row.push('#');
                    }
                    '.' => {
                        new_row.push('.');
                        new_row.push('.');
                    }
                    'O' => {
                        new_row.push('[');
                        new_row.push(']');
                    }
                    '@' => {
                        new_row.push('@');
                        new_row.push('.');
                    }
                    _ => panic!("Invalid cell character"),
                }
            }
            widened_map.push(new_row);
        }
        widened_map
    }
}
