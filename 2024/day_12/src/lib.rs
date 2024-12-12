pub mod solutions {
    use std::{collections::HashSet, fs::read_to_string};

    pub fn get_fencing_price(filename: &String) -> u32 {
        let content = read_to_string(filename).unwrap();

        let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

        let mut price: u32 = 0;

        let mut visited: HashSet<(isize, isize)> = HashSet::new();

        for (row_idx, line) in grid.iter().enumerate() {
            for (col_idx, &ch) in line.iter().enumerate() {
                let (area, perimeter) = flood_fill(
                    (row_idx as isize, col_idx as isize),
                    &grid,
                    ch,
                    &mut visited,
                );
                price += area * perimeter;
            }
        }

        price
    }

    fn flood_fill(
        pos: (isize, isize),
        grid: &Vec<Vec<char>>,
        ch: char,
        visited: &mut HashSet<(isize, isize)>,
    ) -> (u32, u32) {
        let mut area: u32 = 1;
        let mut perimeter: u32 = 0;
        if out_of_bounds(pos, grid) || grid[pos.0 as usize][pos.1 as usize] != ch {
            return (0, 1);
        }

        if !visited.insert(pos) {
            return (0, 0);
        }

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (new_area, new_perimeter) = flood_fill((pos.0 + dx, pos.1 + dy), grid, ch, visited);
            area += new_area;
            perimeter += new_perimeter;
        }
        (area, perimeter)
    }

    #[inline]
    fn out_of_bounds(pos: (isize, isize), grid: &Vec<Vec<char>>) -> bool {
        pos.0 < 0 || pos.1 < 0 || pos.0 >= grid.len() as isize || pos.1 >= grid[0].len() as isize
    }
}
