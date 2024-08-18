pub mod seeds {
    use std::{error::Error, fs};

    pub fn part_1(file_name: &String) -> Result<i64, Box<dyn Error>> {
        let content = fs::read_to_string(file_name)?;

        let seeds = get_seeds(&content);

        let maps = get_maps(&content);

        let mut locations: Vec<i64> = Vec::new();

        for mut loc in seeds {
            for map in &maps {
                'map_section: for &(destination, source, range) in map {
                    if source <= loc && loc < source + range {
                        let move_by = destination - source;
                        loc += move_by;
                        break 'map_section;
                    }
                }
            }
            locations.push(loc);
        }

        let min_location = *locations.iter().min().unwrap();

        Ok(min_location)
    }

    pub fn part_2(file_name: &String) -> Result<i64, Box<dyn Error>> {
        let content = fs::read_to_string(file_name)?;

        let seeds_range = get_seeds_range(&content);

        let maps = get_maps(&content);

        let mut locations: Vec<i64> = Vec::new();

        for seed_extremes in seeds_range {
            let mut to_transform_ranges: Vec<(i64, i64)> = vec![seed_extremes];
            // This is the starting range at the begninning of each map.
            // Before we do any transformation at all, this would obviously be the beginning seed range.
            // Later, depending on the overlaps, this would potentially have more than one range.
            // It is possible, that this vector has a range where it partially gets a transformation.
            // So the part that gets the transformation moves to after_transformation_range, while
            // the part that does not get it, moves back to to_transform_ranges.

            let mut after_transformation_ranges: Vec<(i64, i64)> = Vec::new();
            // This is to keep all the partial or full ranges that we get after the transformation.

            for map in &maps {
                while !to_transform_ranges.is_empty() {
                    // initial_range holds the range before the transformation.
                    // Will hold seed range for first map, then the soil range, then fertilizer and so on.
                    let initial_range = to_transform_ranges.pop().unwrap();
                    let mut is_overlapping = false;
                    // This is a flag that checks if a given range has any overlap, with the entire map.
                    // So all the transformations in a given map.

                    'transformation_section: for &(destination, source, range) in map {
                        if initial_range.0 >= source + range || initial_range.1 < source {
                            // No overlap case. Go check next transformation range.
                            continue 'transformation_section;
                        // Doesn't match with this particular transformation.
                        // But this doesn't mean that it goes unchanged.
                        // Possibly, some other transformation works.
                        // So can't push it to after_transformation_range already. :'(
                        // Made this mistake and thus failed :/
                        } else if initial_range.0 >= source && initial_range.1 < source + range {
                            // Full overlap case. Seed range entirely inside the transformation range.
                            is_overlapping = true;
                            let move_by = destination - source;
                            after_transformation_ranges
                                .push((initial_range.0 + move_by, initial_range.1 + move_by));
                            // Once we have found a transformation that works,
                            // the ranges have been modified. And we need to go again with the
                            // new set of ranges.
                            break 'transformation_section;
                        } else if initial_range.0 < source && initial_range.1 < source + range {
                            // Left side of seed range is out of transformation range but right side is in range.
                            // Partial transformation of latter part of the seed range.
                            is_overlapping = true;
                            let overlap_len = initial_range.1 - source;
                            after_transformation_ranges
                                .push((destination, destination + overlap_len)); // This is transformed.
                            to_transform_ranges.push((initial_range.0, source - 1)); // This is what is left
                            break 'transformation_section;
                        } else if initial_range.0 >= source && initial_range.1 > source + range {
                            is_overlapping = true;
                            let overlap_len = source + range - initial_range.0;
                            to_transform_ranges.push((source + range, initial_range.1));
                            after_transformation_ranges
                                .push((destination + range - overlap_len, destination + range));
                            break 'transformation_section;
                        } else if initial_range.0 < source && initial_range.1 >= source + range {
                            is_overlapping = true;
                            to_transform_ranges.push((initial_range.0, source - 1));
                            to_transform_ranges.push((source + range, initial_range.1));
                            after_transformation_ranges.push((destination, destination + range));
                            break 'transformation_section;
                        }
                    }
                    // If when we are done with all the transformation sections, and we still have
                    // no match, it means that these set of seeds go untransformed by this type
                    // of mapping.
                    if !is_overlapping {
                        after_transformation_ranges.push((initial_range.0, initial_range.1));
                    }
                }
            }
            // When we are done with all maps, what is left in the "after_transformation_range"
            // is going to be the locations basically.
            for item in after_transformation_ranges {
                locations.push(item.0);
            }
        }

        let min_location = *locations.iter().min().unwrap();

        Ok(min_location)
    }

    pub fn part_2_alt(file_name: &String) -> Result<i64, Box<dyn Error>> {
        let content = fs::read_to_string(file_name)?;

        let seeds_range = get_seeds_range(&content);

        let maps = get_maps(&content);

        let mut locations: Vec<i64> = Vec::new();

        for seed_extremes in seeds_range {
            let mut to_transform_ranges: Vec<(i64, i64)> = vec![seed_extremes];

            let mut after_transform_ranges: Vec<(i64, i64)> = Vec::new();

            for map in &maps {
                while !to_transform_ranges.is_empty() {
                    let initial_range = to_transform_ranges.pop().unwrap();
                    let mut is_transformed: bool = false;
                    'potential_transforms: for &(destination, source, range) in map {
                        if initial_range.1 < source || initial_range.0 >= source + range {
                            continue 'potential_transforms;
                        } else if initial_range.0 >= source && initial_range.1 < source + range {
                            // Fully inside. Does a full transformation. C1.
                            is_transformed = true;
                            let move_by = destination - source;
                            after_transform_ranges
                                .push((initial_range.0 + move_by, initial_range.1 + move_by));
                            break 'potential_transforms;
                        } else if initial_range.0 < source && initial_range.1 < source + range {
                            // C2. Left side out. Right side within domain.
                            is_transformed = true;
                            let extend_by = initial_range.1 - source;
                            to_transform_ranges.push((initial_range.0, source - 1));
                            after_transform_ranges.push((destination, destination + extend_by));
                            break 'potential_transforms;
                        } else if initial_range.0 >= source && initial_range.1 >= source + range {
                            // C3. Left side in. Right side out.
                            is_transformed = true;
                            let move_by = destination - source;
                            to_transform_ranges.push((source + range, initial_range.1));
                            after_transform_ranges
                                .push((initial_range.0 + move_by, destination + range - 1));
                            break 'potential_transforms;
                        } else if initial_range.0 < source && initial_range.1 >= source + range {
                            // C4. It completely engulfs the domain.
                            is_transformed = true;
                            to_transform_ranges.push((initial_range.0, source - 1));
                            to_transform_ranges.push((source + range, initial_range.1));
                            after_transform_ranges.push((destination, destination + range - 1));
                            break 'potential_transforms;
                        }
                    }

                    if !is_transformed {
                        after_transform_ranges.push((initial_range.0, initial_range.1));
                    }
                }
                to_transform_ranges = after_transform_ranges.clone();
                after_transform_ranges = Vec::new();
            }
            for item in to_transform_ranges {
                locations.push(item.0);
            }
        }
        let min_location = *locations.iter().min().unwrap();

        Ok(min_location)
    }

    fn get_seeds(content: &String) -> Vec<i64> {
        let mut seeds: Vec<i64> = Vec::new();
        for seed in content
            .lines()
            .nth(0)
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
        {
            seeds.push(seed.parse::<i64>().unwrap())
        }
        seeds
    }

    fn get_maps(content: &String) -> Vec<Vec<(i64, i64, i64)>> {
        let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();

        let mut current_map: Vec<(i64, i64, i64)> = Vec::new();

        for line in content.lines().skip(2) {
            if line.is_empty() {
                if current_map.is_empty() {
                    continue; // If the current map is empty, and we find an empty line, move on.
                } else {
                    maps.push(current_map); // If the current map is not empty, and we find an empty line, that means we are done with this kind of map.
                    current_map = Vec::new();
                }
            } else {
                if line.contains(":") {
                    continue; // This is the title of the type of map.
                              // Maybe I should assert here to check that at this point, current map is empty? IDK.
                } else {
                    let mut nums: Vec<i64> = Vec::new();

                    for num in line.split_whitespace() {
                        nums.push(num.parse().unwrap());
                    }

                    current_map.push((nums[0], nums[1], nums[2]));
                }
            }
        }

        if !current_map.is_empty() {
            // An empty line marks the end of a map, except if it is the last map, so doing this here.
            maps.push(current_map);
        }

        maps
    }

    fn get_seeds_range(content: &String) -> Vec<(i64, i64)> {
        let mut seeds_range: Vec<(i64, i64)> = Vec::new();

        let seeds = get_seeds(content);

        let mut i = 0;

        while i + 1 < seeds.len() {
            seeds_range.push((seeds[i], seeds[i] + seeds[i + 1] - 1));
            i += 2;
        }

        seeds_range
    }
}

// Plan for part 2:
// Basically, in the range of seeds, there can either be no overlap, full overlap, or partial overlap.
// If no or full overlap, then we can move aheaad with the transformation.
// In case there is partial overlap, we need to split the input range into two parts.
// And then do relevant transformations for both of them.
// Need to handle this carefully. Especially the boundary terms I think.
//
// Steps to take:
// 1. Read the seed ranges -> convert them to seed min and max instead of this start and range thingy.
// 2. For each range, go through the transformations.
// 3. If during the transformations, we need to split it, because of the partial overlap thing, then we get 2 ranges basically.
// 4. Continue till the end (might be more splitting off potentially).
// 5. In the location vector, to be able to do min later, I think I can store the final address of all the seeds?
// Or alternatively, I can store the location of only the extreme of the ranges (after all the splitting has been done internally?).
