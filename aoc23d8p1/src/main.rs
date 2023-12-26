use std::fs;
use std::collections::HashMap;

use regex::Regex;

struct Node {
    l: String,
    r: String,
}

// Totally unnecessary? Makes code later cleaner I think.
impl Node {
    fn get(&self, side: char) -> &String {
        if side == 'L' {
            &self.l
        } else {
            &self.r
        }
    }
}

// For infinitely iterating over the LR string.

/*  
    UPDATE: wait, doesn't .chars() on String already do this?
    lmao I can't believe I pushed this. Fwiw I was up until
    2am last night and did this before caffeine had kicked in.
    Sorry, still new at Rust.
*/
struct Consumer {
    sides: Vec<char>,
    i: usize,
}

impl Consumer {
    fn new(orly: Vec<char>) -> Consumer {
        Consumer { 
            sides: orly,
            i: 0,
        }
    }
}

impl Iterator for Consumer {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {

        if &self.i >= &self.sides.len() {
            self.i = 0;
        }

        let side = &self.sides[self.i];
        self.i += 1;

        Some(*side)
    }
}

fn main() {

    let mut network_map: HashMap<String,Node> = HashMap::new();
    let mut last = String::from("AAA");
    let mut orly: Vec<char> = Vec::new();

    for (i, line) in fs::read_to_string("input.txt")
            .unwrap().lines().enumerate() {

        if i == 0 {
            orly = line.chars().collect();
            continue;
        }

        if !line.contains("=") {
            continue;
        }

        let re = Regex::new(r"([A-Z])\w+").unwrap();
        let directions: Vec<&str> = re.find_iter(line)
            .map(|m| m.as_str()).collect();

        network_map.insert(directions[0].to_string(), Node { 
            l: directions[1].to_string(),
            r: directions[2].to_string(),
        });
    }

    let mut steps: i32 = 0;
    let mut orlynow = Consumer::new(orly);
    loop {
        steps += 1;
        let next_side = orlynow.next().unwrap();
        last = network_map[&last].get(next_side).clone();
        if last == "ZZZ".to_string() {
            println!("{}", steps);
            break;
        }
    }
}
