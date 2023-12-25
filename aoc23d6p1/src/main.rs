use std::fs;
use regex::Regex;

fn main() {

    let mut lines: Vec<Vec<i64>> = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();
    let mut advent: Vec<i64> = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        lines.push(
            re.find_iter(line)
                    .filter_map(|digits| digits.as_str().parse().ok())
                    .collect()
        );
    }

    for i in 0..lines[0].len() {
        let time = lines[0][i];
        let distance = lines[1][i];
        let mut winning: i64 = 0;

        for j in 1..=time {
            if j * (time - j) > distance {
                winning += 1;
            }
        }

        advent.push(winning);
    }

    println!("{:?}", advent);
    println!("{:?}", advent.iter().product::<i64>());
}
