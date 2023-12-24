// Rust doesn't recommend recursion. Well... okay.

use std::fs;
use regex::Regex;

struct Advent {
    destination: i64,
    source: i64,
    range: i64,
}

impl Advent {
    fn in_range(&self, seed: i64) -> bool {
        if seed >= self.source && seed < (self.source + self.range) {
            return true
        } else {
            return false
        }
    }
    fn correspondence(&self, seed: i64) -> i64 {
        let i = seed - self.source;
        return self.destination + i
    }
}

fn dispense_numbers(re: &Regex, line: &str) -> Advent {
    let numbers: Vec<i64> = re.find_iter(line)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect();
    Advent {
        destination: numbers[0],
        source: numbers[1],
        range: numbers[2],
    }
}

fn return_map_matches(mut seed: i64, not_a_map: &Vec<Vec<Advent>>) -> i64 {

    for x in not_a_map {
        for y in x {
            if y.in_range(seed) {
                seed = y.correspondence(seed);
                break;
            }
        }
    }

    seed

}

fn main() {

    let re = Regex::new(r"(\d+)").unwrap();
    let mut seeds: Vec<i64> = Vec::new();
    let mut lowest: Vec<i64> = Vec::new();
    // Since we're traversing in order, this makes more sense
    // than a hash map.
    let mut advent_dump: Vec<Vec<Advent>> = Vec::new();
    advent_dump.push(Vec::new());
    let mut i: usize = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        if line.contains("seeds:") {
            seeds = {
                re.find_iter(line)
                    .filter_map(|digits| digits.as_str().parse().ok())
                    .collect()
            }
        } else if line.contains("map:") && !line.contains("seed-to-soil") {
            i += 1;
            advent_dump.push(Vec::new());
        } else if re.is_match(line) {
            advent_dump[i].push(dispense_numbers(&re, line));
        }
    }

    for seed in seeds {
        lowest.push(return_map_matches(seed, &advent_dump));
    }

    println!("{}", lowest.iter().min().unwrap());
}
