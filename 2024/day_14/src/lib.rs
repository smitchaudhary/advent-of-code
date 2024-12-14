pub mod solutions {
    use std::fs::read_to_string;

    pub fn get_safety_factor(filename: &String, size: Vec<i32>) -> u32 {
        let content = read_to_string(filename).unwrap();

        let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);

        for line in content.lines() {
            let mut split = line.split(" ");
            let mut p: Vec<i32> = split.next().unwrap()[2..]
                .split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let v: Vec<i32> = split.next().unwrap()[2..]
                .split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            for i in 0..2 {
                p[i] = (p[i] + v[i] * 100) % size[i];
                if p[i] < 0 {
                    p[i] += size[i];
                }
            }

            if p[0] == size[0] / 2 || p[1] == size[1] / 2 {
                continue;
            } else if p[0] < size[0] / 2 && p[1] < size[1] / 2 {
                a += 1;
            } else if p[0] > size[0] / 2 && p[1] < size[1] / 2 {
                b += 1;
            } else if p[0] < size[0] / 2 && p[1] > size[1] / 2 {
                c += 1;
            } else {
                d += 1;
            }
        }
        a * b * c * d
    }
}
