pub mod lava {
    use std::{cmp::min, fs};

    pub fn run(file_name: &String) -> u32 {
        let all_grids = fs::read_to_string(file_name)
            .unwrap()
            .split("\n\n")
            .map(|grid| {
                grid.lines()
                    .map(|line| line.chars().collect())
                    .collect::<Vec<Vec<char>>>()
            })
            .collect::<Vec<Vec<Vec<char>>>>();

        let mut grid_summary = 0;

        for grid in all_grids {
            grid_summary += get_grid_summary(grid);
        }
        grid_summary
    }

    fn get_grid_summary(grid: Vec<Vec<char>>) -> u32 {
        for mirror_idx in 1..(grid[0].len() - 1) {
            if is_mirror_index(&grid, true, mirror_idx) {
                return mirror_idx as u32 + 1;
            }
        }

        for mirror_idx in 1..(grid.len() - 1) {
            if is_mirror_index(&grid, false, mirror_idx) {
                return 100 * (mirror_idx as u32 + 1);
            }
        }
        unreachable!()
    }

    fn is_mirror_index(grid: &Vec<Vec<char>>, vertical: bool, mirror_idx: usize) -> bool {
        let num_reflected_lines = match vertical {
            true => grid.len(),
            false => grid[0].len(),
        };

        let orthogonal_dist = match vertical {
            true => grid[0].len(),
            false => grid.len(),
        };

        let num_lines_in_view = min(mirror_idx + 1, orthogonal_dist - mirror_idx - 1);

        println!("Vertical is: {}", vertical);
        println!("Mirror index: {}", mirror_idx);
        println!("Number of lines in view: {}", num_lines_in_view);

        for i in 0..num_reflected_lines {
            println!("Checking the line {} for mirror", i);
            for j in 0..num_lines_in_view {
                println!("For items {} away", j);
                let pos = match vertical {
                    true => (mirror_idx - j, i),
                    false => (i, mirror_idx - j),
                };

                let reflected_pos = match vertical {
                    true => (mirror_idx + j + 1, i),
                    false => (i, mirror_idx + j + 1),
                };

                println!(
                    "Position {}, {} and reflected to {}, {}",
                    pos.0, pos.1, reflected_pos.0, reflected_pos.1
                );
                if grid[pos.1][pos.0] != grid[reflected_pos.1][reflected_pos.0] {
                    return false;
                }
            }
        }
        true
    }
}
