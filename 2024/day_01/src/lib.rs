pub mod solutions {
    use std::fs::read_to_string;

    pub fn distance(filename: &str) -> u32 {
        let (mut col1, mut col2): (Vec<u32>, Vec<u32>) = get_cols(filename);

        col1.sort();
        col2.sort();

        let mut distance: u32 = 0;

        for (i, j) in col1.into_iter().zip(col2.into_iter()) {
            distance += i.abs_diff(j);
        }

        distance
    }

    pub fn similarity_score(filename: &str) -> u32 {
        let (col1, col2): (Vec<u32>, Vec<u32>) = get_cols(filename);
        let mut score = 0;

        for i in col1 {
            score += i * col2.iter().copied().filter(|&j| j == i).count() as u32;
        }
        score
    }

    fn get_cols(filename: &str) -> (Vec<u32>, Vec<u32>) {
        read_to_string(filename)
            .unwrap()
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<&str>>())
            .map(|pair| {
                (
                    pair[0].parse::<u32>().unwrap(),
                    pair[1].parse::<u32>().unwrap(),
                )
            })
            .unzip()
    }
}
