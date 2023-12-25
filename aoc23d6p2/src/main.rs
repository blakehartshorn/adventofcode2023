use std::fs;
use regex::Regex;

fn main() {

    let mut lines: Vec<i64> = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let re = Regex::new(r"[^\d]").unwrap();
        let number_str = re.replace_all(line, "").parse();
        match number_str {
            Ok(number) => lines.push(number),
            Err(_) => continue,
        };
    }

    let mut winning: i64 = 0;
    let time = lines[0];
    let distance = lines[1];

    for j in 1..=time {
        if j * (time - j) > distance {
            winning += 1;
        }
    }

    println!("{winning}");
}
