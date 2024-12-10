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

    pub fn get_checksum_block_moves(filename: &String) -> u64 {
        let content = read_to_string(filename).unwrap();
        let map: Vec<u64> = content
            .trim()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();

        let disk_size = map.iter().sum::<u64>() as usize;
        let mut blockmap: Vec<Option<u64>> = Vec::with_capacity(disk_size);

        let mut blocks: Vec<(usize, u64, u64)> = Vec::new();
        let mut gaps: Vec<(u64, usize)> = Vec::new();

        let mut current_pos = 0;
        for (idx, &val) in map.iter().enumerate() {
            if idx % 2 == 0 {
                blocks.push((current_pos, (idx / 2) as u64, val));
                blockmap.extend(iter::repeat(Some((idx / 2) as u64)).take(val as usize));
            } else {
                gaps.push((val, current_pos));
                blockmap.extend(iter::repeat(None).take(val as usize));
            }
            current_pos += val as usize;
        }

        for block in blocks.iter().rev() {
            let (position, id, length) = *block;

            let mut gap_idx = 0;
            while gap_idx < gaps.len() {
                let (gap_length, gap_position) = gaps[gap_idx];

                if gap_position > position {
                    break;
                }

                if gap_length >= length {
                    for l in 0..length as usize {
                        blockmap[position + l] = None;
                        blockmap[gap_position + l] = Some(id);
                    }

                    let diff = gap_length - length;
                    if diff > 0 {
                        gaps[gap_idx] = (diff, gap_position + length as usize);
                    } else {
                        gaps.remove(gap_idx);
                    }
                    break;
                }
                gap_idx += 1;
            }
        }

        blockmap
            .iter()
            .enumerate()
            .map(|(idx, &num)| match num {
                Some(val) => (idx as u64) * val,
                None => 0,
            })
            .sum()
    }
}
