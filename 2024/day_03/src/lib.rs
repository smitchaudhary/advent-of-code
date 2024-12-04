pub mod solution {
    use std::fs::read_to_string;

    use regex::Regex;

    pub fn mul_total(filename: &String) -> u32 {
        let contents = read_to_string(filename).unwrap();

        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let muls = re.captures_iter(&contents).map(|pair| {
            let a: u32 = pair[1].parse().unwrap();
            let b: u32 = pair[2].parse().unwrap();
            a * b
        });

        // let a = re.captures_iter(&contents);
        // for cap in a {
        //     let cc = &cap[2];
        //     println!("{:?}", cap);
        //     println!("{:?}", cc);
        // }

        muls.sum()
    }

    pub fn mul_total_do_dont(filename: &String) -> u32 {
        let contents = read_to_string(filename).unwrap();

        let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let do_re = Regex::new(r"do\(\)").unwrap();
        let dont_re = Regex::new(r"don't\(\)").unwrap();

        let mut control_locations: Vec<(usize, bool)> = Vec::new();
        for mat in do_re.find_iter(&contents) {
            control_locations.push((mat.start(), true));
        }
        for mat in dont_re.find_iter(&contents) {
            control_locations.push((mat.start(), false));
        }

        control_locations.sort_by_key(|&(index, _)| index);

        let mut enabled_regions: Vec<(usize, usize)> = Vec::new();
        let mut is_enabled: bool = true;
        let mut last_index: usize = 0;

        let mut mul_results: u32 = 0;

        for (idx, is_do) in control_locations {
            if is_enabled {
                enabled_regions.push((last_index, idx));
            }

            is_enabled = is_do;
            last_index = idx;
        }

        if is_enabled {
            enabled_regions.push((last_index, contents.len()));
        }

        for mat in mul_re.find_iter(&contents) {
            let mul_start = mat.start();
            if enabled_regions
                .iter()
                .any(|&(start, end)| mul_start >= start && mul_start < end)
            {
                let cap = mul_re.captures(mat.as_str()).unwrap();
                mul_results += cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap();
            }
        }

        mul_results
    }
}
