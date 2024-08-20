pub mod galaxy {
    use std::fs;

    pub fn run(file_name: &String, expansion_scale: u128) -> u128 {
        let content = fs::read_to_string(file_name).unwrap();

        let mut galaxy_locations: Vec<(usize, usize)> = Vec::new();

        let mut rows_with_galaxy: Vec<usize> = Vec::new();
        let mut cols_with_galaxy: Vec<usize> = Vec::new();

        for (row, line) in content.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    galaxy_locations.push((row, col));
                    rows_with_galaxy.push(row);
                    cols_with_galaxy.push(col);
                }
            }
        }
        rows_with_galaxy.dedup();
        cols_with_galaxy.dedup();

        let mut total_min_dist = 0;
        for i in 0..galaxy_locations.len() {
            for j in (i + 1)..galaxy_locations.len() {
                let the_dist = min_dist(
                    galaxy_locations[i],
                    galaxy_locations[j],
                    &rows_with_galaxy,
                    &cols_with_galaxy,
                    expansion_scale,
                );
                total_min_dist += the_dist;
            }
        }

        total_min_dist
    }

    fn min_dist(
        loc_1: (usize, usize),
        loc_2: (usize, usize),
        rows_with_galaxy: &Vec<usize>,
        cols_with_galaxy: &Vec<usize>,
        expansion_scale: u128,
    ) -> u128 {
        let mut dist = 0;

        let (row1, row2) = if loc_1.0 < loc_2.0 {
            (loc_1.0, loc_2.0)
        } else {
            (loc_2.0, loc_1.0)
        };

        let (col1, col2) = if loc_1.1 < loc_2.1 {
            (loc_1.1, loc_2.1)
        } else {
            (loc_2.1, loc_1.1)
        };

        for i in row1..row2 {
            dist += if rows_with_galaxy.contains(&i) {
                1
            } else {
                expansion_scale
            }
        }

        for i in col1..col2 {
            dist += if cols_with_galaxy.contains(&i) {
                1
            } else {
                expansion_scale
            }
        }

        dist
    }
}
