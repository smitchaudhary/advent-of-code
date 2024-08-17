pub mod map {
    use std::{error::Error, fs};

    pub fn part_1(file_name: &String) -> Result<u32, Box<dyn Error>> {
        let content = &fs::read_to_string(file_name)?;

        let mut num_vec: Vec<NumberOnMap> = Vec::new();

        let mut max_col: usize = 0;
        let max_row = content.lines().count() - 1;

        if let Some(first_line) = content.lines().next() {
            max_col = first_line.chars().count() - 1;
        }

        for (row, line) in content.lines().enumerate() {
            add_nums_to_vec(line, row, &mut num_vec);
        }

        let mut total = 0;

        for item in &num_vec {
            if item.is_part_num(content, max_row, max_col) {
                total += item.num;
            }
        }

        Ok(total)
    }

    pub fn part_2(file_name: &String) -> Result<u32, Box<dyn Error>> {
        let content = &fs::read_to_string(file_name)?;

        let mut num_vec: Vec<NumberOnMap> = Vec::new();

        let mut max_col: usize = 0;
        let max_row = content.lines().count() - 1;

        if let Some(first_line) = content.lines().next() {
            max_col = first_line.chars().count() - 1;
        }

        for (row, line) in content.lines().enumerate() {
            add_nums_to_vec(line, row, &mut num_vec);
        }

        let mut gear_sum: u32 = 0;
        // Currently, this checks for all the pairs.
        // No need for this.
        // Should only check for those numbers which are in the "vicinity".
        // Anything more than 2 lines away is not going to be a valid one.
        for i in 0..num_vec.len() {
            if let Some(star_idx1) = num_vec[i].get_star_index(content, max_row, max_col) {
                for j in (i + 1)..num_vec.len() {
                    if let Some(star_idx2) = num_vec[j].get_star_index(content, max_row, max_col) {
                        if star_idx1 == star_idx2 {
                            gear_sum += num_vec[i].num * num_vec[j].num;
                        }
                    }
                }
            }
        }

        Ok(gear_sum)
    }

    struct NumberOnMap {
        num: u32,
        row: usize,
        start_idx: usize,
        length: usize,
    }

    impl NumberOnMap {
        fn is_part_num(&self, content: &String, max_row: usize, max_col: usize) -> bool {
            let indices_to_check = self.get_indices_to_check(max_row, max_col);
            for (row, col) in indices_to_check {
                if let Some(ch) = content.lines().nth(row).unwrap().chars().nth(col) {
                    if ch != '.' {
                        return true;
                    }
                }
            }
            false
        }

        fn get_star_index(
            &self,
            content: &String,
            max_row: usize,
            max_col: usize,
        ) -> Option<(usize, usize)> {
            let star_index = None;

            let indices_to_check = self.get_indices_to_check(max_row, max_col);
            for (row, col) in indices_to_check {
                if let Some(ch) = content.lines().nth(row).unwrap().chars().nth(col) {
                    if ch == '*' {
                        return Some((row, col));
                    }
                }
            }
            star_index
        }

        fn get_indices_to_check(&self, max_row: usize, max_col: usize) -> Vec<(usize, usize)> {
            let mut indices = Vec::new();

            if self.row != 0 {
                if self.start_idx != 0 {
                    indices.push((self.row - 1, self.start_idx - 1));
                }
                if self.start_idx + self.length < max_col - 1 {
                    indices.push((self.row - 1, self.start_idx + self.length))
                }
                for col in self.start_idx..(self.start_idx + self.length) {
                    indices.push((self.row - 1, col));
                }
            }

            if self.row != max_row {
                if self.start_idx != 0 {
                    indices.push((self.row + 1, self.start_idx - 1));
                }
                if self.start_idx + self.length < max_col - 1 {
                    indices.push((self.row + 1, self.start_idx + self.length))
                }
                for col in self.start_idx..(self.start_idx + self.length) {
                    indices.push((self.row + 1, col));
                }
            }
            if self.start_idx != 0 {
                indices.push((self.row, self.start_idx - 1));
            }
            if self.start_idx + self.length < max_col - 1 {
                indices.push((self.row, self.start_idx + self.length))
            }
            // I clearly don't know how to use closures. Next chapter in the book.
            // Maybe revisit this afterwards.
            indices
        }
    }

    fn add_nums_to_vec<'a>(
        line: &'a str,
        row: usize,
        num_vec: &'a mut Vec<NumberOnMap>,
    ) -> &'a Vec<NumberOnMap> {
        let mut start_index: Option<usize> = None;

        for (idx, ch) in line.char_indices() {
            if ch.is_digit(10) {
                if start_index.is_none() {
                    start_index = Some(idx);
                } else {
                    if idx == line.len() - 1 {
                        if let Some(start_idx) = start_index {
                            let num = line[start_idx..=idx].parse::<u32>().unwrap();
                            num_vec.push(NumberOnMap {
                                num,
                                row,
                                start_idx,
                                length: idx + 1 - start_idx,
                            });
                        }
                    }
                }
            } else {
                if let Some(start_idx) = start_index {
                    let num = line[start_idx..idx].parse::<u32>().unwrap();
                    num_vec.push(NumberOnMap {
                        num,
                        row,
                        start_idx,
                        length: idx - start_idx,
                    });
                    start_index = None;
                }
            }
        }
        num_vec
    }
}
