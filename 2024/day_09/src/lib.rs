pub mod solutions {
    use std::{fs::read_to_string, iter};

    pub fn get_checksum(filename: &String) -> u64 {
        let content = read_to_string(filename).unwrap();

        let map: Vec<u64> = content
            .trim()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();

        let disk_size = map.iter().sum::<u64>() as usize;

        let mut blockmap: Vec<Option<u64>> = Vec::with_capacity(disk_size);

        for (idx, &block) in map.iter().enumerate() {
            if idx % 2 == 0 {
                blockmap.extend(iter::repeat(Some((idx / 2) as u64)).take(block as usize));
            } else {
                blockmap.extend(iter::repeat(None).take(block as usize));
            }
        }

        loop {
            while blockmap.last().unwrap() == &None {
                blockmap.pop();
            }

            match blockmap.iter().position(|&x| x == None) {
                Some(next_free) => blockmap[next_free] = Some(blockmap.pop().unwrap().unwrap()),
                None => break,
            }
        }
        blockmap
            .iter()
            .filter(|&&x| x != None)
            .enumerate()
            .map(|(idx, &num)| (idx as u64) * num.unwrap())
            .sum::<u64>()
    }
}
