pub mod solutions {
    use std::fs::read_to_string;

    pub fn part_1(filename: &String) -> u32 {
        let contents = read_to_string(filename).unwrap();

        let grid: Vec<Vec<char>> = get_grid(&contents);

        let mut xmas_count: u32 = 0;

        xmas_count += count_word_in_grid(&grid, "XMAS");
        xmas_count += count_word_in_grid(&grid, "SAMX");

        let transposed_grid = transpose_grid(&grid);

        xmas_count += count_word_in_grid(&transposed_grid, "XMAS");
        xmas_count += count_word_in_grid(&transposed_grid, "SAMX");

        let top_left_to_bottom_right_diag = get_top_left_to_bottom_right_diagonals(&grid);

        xmas_count += count_word_in_grid(&top_left_to_bottom_right_diag, "XMAS");
        xmas_count += count_word_in_grid(&top_left_to_bottom_right_diag, "SAMX");

        let top_right_to_bottom_left_grid = get_top_right_to_bottom_left_diagonals(&grid);

        xmas_count += count_word_in_grid(&top_right_to_bottom_left_grid, "XMAS");
        xmas_count += count_word_in_grid(&top_right_to_bottom_left_grid, "SAMX");

        xmas_count
    }

    pub fn part_2(filename: &String) -> u32 {
        let contents = read_to_string(filename).unwrap();
        let grid: Vec<Vec<char>> = get_grid(&contents);
        
        let rows = grid.len();
        let cols = grid[0].len();
        let mut xmas_count = 0;

        // Check each possible center point of the X
        for i in 1..rows-1 {
            for j in 1..cols-1 {
                if grid[i][j] == 'A' {
                    // Check all possible combinations of MAS (forward/backward) in X shape
                    // Top-left to bottom-right and top-right to bottom-left
                    if is_xmas_pattern(&grid, i, j) {
                        xmas_count += 1;
                    }
                }
            }
        }

        xmas_count
    }

    fn get_grid(contents: &String) -> Vec<Vec<char>> {
        contents
            .lines()
            .map(|line| line.chars().collect())
            .collect()
    }

    fn transpose_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        (0..grid[0].len())
            .map(|col| grid.iter().map(|row| row[col]).collect())
            .collect()
    }

    fn count_word_in_grid(grid: &Vec<Vec<char>>, word: &str) -> u32 {
        grid.iter()
            .map(|line| line.iter().collect::<String>())
            .map(|line| line.matches(word).count() as u32)
            .sum()
    }

    fn get_top_right_to_bottom_left_diagonals(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut diagonals = Vec::new();

        // Main diagonals
        for k in 0..rows + cols - 1 {
            let mut diagonal = Vec::new();

            for i in 0..rows {
                if k + 1 + i >= rows {
                    let j = k - (rows - 1 - i);
                    if j < cols {
                        diagonal.push(grid[i][j]);
                    }
                }
            }

            if diagonal.len() >= 4 {
                // Only keep diagonals of length 4 or more
                diagonals.push(diagonal);
            }
        }

        diagonals
    }

    fn get_top_left_to_bottom_right_diagonals(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut diagonals = Vec::new();

        // Main diagonals
        for k in 0..rows + cols - 1 {
            let mut diagonal = Vec::new();

            for i in 0..rows {
                if k >= i {
                    let j = k - i;
                    if j < cols {
                        diagonal.push(grid[i][j]);
                    }
                }
            }

            if diagonal.len() >= 4 {
                // Only keep diagonals of length 4 or more
                diagonals.push(diagonal);
            }
        }

        diagonals
    }

    fn is_xmas_pattern(grid: &Vec<Vec<char>>, center_i: usize, center_j: usize) -> bool {
        // Check diagonal patterns
        let tl_mas = check_mas_diagonal(grid, center_i-1, center_j-1, -1, -1);
        let tr_mas = check_mas_diagonal(grid, center_i-1, center_j+1, -1, 1);
        let bl_mas = check_mas_diagonal(grid, center_i+1, center_j-1, 1, -1);
        let br_mas = check_mas_diagonal(grid, center_i+1, center_j+1, 1, 1);

        // Check straight patterns
        let top_mas = check_mas_straight(grid, center_i-1, center_j, -1, 0);
        let bottom_mas = check_mas_straight(grid, center_i+1, center_j, 1, 0);
        let left_mas = check_mas_straight(grid, center_i, center_j-1, 0, -1);
        let right_mas = check_mas_straight(grid, center_i, center_j+1, 0, 1);

        // Combine all patterns
        let diagonal_patterns = [tl_mas, tr_mas, bl_mas, br_mas];
        let straight_patterns = [top_mas, bottom_mas, left_mas, right_mas];
        
        // Check if we have exactly two valid patterns in either diagonal or straight configuration
        let valid_diagonal_patterns: Vec<_> = diagonal_patterns.iter().filter(|&&x| x).collect();
        let valid_straight_patterns: Vec<_> = straight_patterns.iter().filter(|&&x| x).collect();
        
        valid_diagonal_patterns.len() == 2 || valid_straight_patterns.len() == 2
    }

    fn check_mas_diagonal(grid: &Vec<Vec<char>>, i: usize, j: usize, di: i32, dj: i32) -> bool {
        let next_i = i as i32 + di;
        let next_j = j as i32 + dj;
        
        if next_i < 0 || next_j < 0 || next_i >= grid.len() as i32 || next_j >= grid[0].len() as i32 {
            return false;
        }

        // Check for "MAS" (forward) or "SAM" (backward)
        let forward = grid[i][j] == 'M' && grid[next_i as usize][next_j as usize] == 'S';
        let backward = grid[i][j] == 'S' && grid[next_i as usize][next_j as usize] == 'M';
        
        forward || backward
    }

    fn check_mas_straight(grid: &Vec<Vec<char>>, i: usize, j: usize, di: i32, dj: i32) -> bool {
        let next_i = i as i32 + di;
        let next_j = j as i32 + dj;
        
        if next_i < 0 || next_j < 0 || next_i >= grid.len() as i32 || next_j >= grid[0].len() as i32 {
            return false;
        }

        // Check for "MAS" (forward) or "SAM" (backward)
        let forward = grid[i][j] == 'M' && grid[next_i as usize][next_j as usize] == 'S';
        let backward = grid[i][j] == 'S' && grid[next_i as usize][next_j as usize] == 'M';
        
        forward || backward
    }
}
