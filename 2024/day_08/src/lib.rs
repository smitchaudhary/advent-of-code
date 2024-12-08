pub mod solutions {
    use itertools::Itertools;

    use std::{
        collections::{HashMap, HashSet},
        fs::read_to_string,
    };

    pub fn count_antinodes(filename: &String) -> u32 {
        let content = read_to_string(filename).unwrap();

        let grid = content
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let num_rows = grid.len() as isize;
        let num_cols = grid[0].len() as isize;

        let antenas = get_antenas(&grid);
        let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

        for freq in antenas.keys() {
            for (&a, &b) in antenas.get(freq).unwrap().iter().tuple_combinations() {
                let (node1, node2) = get_antinodes(a, b);

                if within_bounds(node1, num_rows, num_cols) {
                    antinodes.insert(node1);
                }
                if within_bounds(node2, num_rows, num_cols) {
                    antinodes.insert(node2);
                }
            }
        }

        antinodes.len() as u32
    }

    pub fn count_resonant_antinodes(filename: &String) -> u32 {
        let content = read_to_string(filename).unwrap();

        let grid = content
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let num_rows = grid.len() as isize;
        let num_cols = grid[0].len() as isize;

        let antenas = get_antenas(&grid);
        let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

        for freq in antenas.keys() {
            for (&a, &b) in antenas.get(freq).unwrap().iter().tuple_combinations() {
                let nodes = get_resonant_antinodes(a, b, num_rows, num_cols);

                for node in nodes {
                    if within_bounds(node, num_rows, num_cols) {
                        antinodes.insert(node);
                    }
                }
            }
        }

        antinodes.len() as u32
    }

    fn get_antenas(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(isize, isize)>> {
        let mut antenas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

        for (row_idx, line) in grid.iter().enumerate() {
            for (col_idx, &ch) in line.iter().enumerate() {
                if ch != '.' {
                    if antenas.contains_key(&ch) {
                        antenas
                            .get_mut(&ch)
                            .unwrap()
                            .push((row_idx as isize, col_idx as isize));
                    } else {
                        antenas.insert(ch, vec![(row_idx as isize, col_idx as isize)]);
                    }
                }
            }
        }

        antenas
    }

    fn get_antinodes(
        pos1: (isize, isize),
        pos2: (isize, isize),
    ) -> ((isize, isize), (isize, isize)) {
        let dx = pos1.0 - pos2.0;
        let dy = pos1.1 - pos2.1;

        let node1 = (pos1.0 + dx, pos1.1 + dy);
        let node2 = (pos2.0 - dx, pos2.1 - dy);

        (node1, node2)
    }

    fn within_bounds(node: (isize, isize), num_rows: isize, num_cols: isize) -> bool {
        if node.0 < 0 || node.0 >= num_rows {
            return false;
        } else if node.1 < 0 || node.1 >= num_cols {
            return false;
        }

        true
    }

    fn get_resonant_antinodes(
        pos1: (isize, isize),
        pos2: (isize, isize),
        num_rows: isize,
        num_cols: isize,
    ) -> Vec<(isize, isize)> {
        let dx = pos1.0 - pos2.0;
        let dy = pos1.1 - pos2.1;

        let mut antinodes: Vec<(isize, isize)> = Vec::new();

        let mut pos = pos1;

        while within_bounds(pos, num_rows, num_cols) {
            antinodes.push(pos);

            pos = (pos.0 + dx, pos.1 + dy);
        }

        pos = pos2;

        while within_bounds(pos, num_rows, num_cols) {
            antinodes.push(pos);

            pos = (pos.0 - dx, pos.1 - dy);
        }

        antinodes
    }
}
