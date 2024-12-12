pub mod solutions {
    use std::{collections::HashSet, fs::read_to_string};

    pub fn get_fencing_price(filename: &String) -> (u32, u32) {
        let content = read_to_string(filename).unwrap();

        let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

        let mut price: u32 = 0;
        let mut discounted_price: u32 = 0;

        let mut visited: HashSet<(isize, isize)> = HashSet::new();

        for (row_idx, line) in grid.iter().enumerate() {
            for (col_idx, &ch) in line.iter().enumerate() {
                let (area, perimeter, sides) = flood_fill(
                    (row_idx as isize, col_idx as isize),
                    &grid,
                    ch,
                    &mut visited,
                );
                price += area * perimeter;
                discounted_price += area * sides;
            }
        }

        (price, discounted_price)
    }

    fn flood_fill(
        pos: (isize, isize),
        grid: &Vec<Vec<char>>,
        ch: char,
        visited: &mut HashSet<(isize, isize)>,
    ) -> (u32, u32, u32) {
        let mut area: u32 = 1;
        let mut perimeter: u32 = 0;
        let mut corners: u32 = 0;

        if out_of_bounds(pos, grid) || grid[pos.0 as usize][pos.1 as usize] != ch {
            return (0, 1, 0);
        }

        if !visited.insert(pos) {
            return (0, 0, 0);
        }

        let directions = [
            [(-1, 0), (0, 1)],  // top-right
            [(0, 1), (1, 0)],   // right-bottom
            [(1, 0), (0, -1)],  // bottom-left
            [(0, -1), (-1, 0)], // left-top
        ];

        for &[dir1, dir2] in directions.iter() {
            let neighbor1 = (pos.0 + dir1.0, pos.1 + dir1.1);
            let neighbor2 = (pos.0 + dir2.0, pos.1 + dir2.1);
            let diagonal = (pos.0 + dir1.0 + dir2.0, pos.1 + dir1.1 + dir2.1);

            // Check for outward corners
            let is_empty1 = out_of_bounds(neighbor1, grid)
                || grid[neighbor1.0 as usize][neighbor1.1 as usize] != ch;
            let is_empty2 = out_of_bounds(neighbor2, grid)
                || grid[neighbor2.0 as usize][neighbor2.1 as usize] != ch;

            // Check for inward corners
            let is_region1 = !out_of_bounds(neighbor1, grid)
                && grid[neighbor1.0 as usize][neighbor1.1 as usize] == ch;
            let is_region2 = !out_of_bounds(neighbor2, grid)
                && grid[neighbor2.0 as usize][neighbor2.1 as usize] == ch;
            let is_diagonal_region = !out_of_bounds(diagonal, grid)
                && grid[diagonal.0 as usize][diagonal.1 as usize] == ch;

            // Count outward corners
            if is_empty1 && is_empty2 {
                corners += 1;
            }

            // Count inward corners
            if is_region1 && is_region2 && !is_diagonal_region {
                corners += 1;
            }
        }

        // Continue with flood fill in all four directions
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (new_area, new_perimeter, new_corners) =
                flood_fill((pos.0 + dx, pos.1 + dy), grid, ch, visited);
            area += new_area;
            perimeter += new_perimeter;
            corners += new_corners;
        }

        (area, perimeter, corners)
    }

    #[inline]
    fn out_of_bounds(pos: (isize, isize), grid: &Vec<Vec<char>>) -> bool {
        pos.0 < 0 || pos.1 < 0 || pos.0 >= grid.len() as isize || pos.1 >= grid[0].len() as isize
    }
}
