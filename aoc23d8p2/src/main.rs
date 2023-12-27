use std::fs;
use std::collections::HashMap;
use num::integer::lcm;
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

fn main() {

    let mut multipliers: Vec<i64> = Vec::new();
    let mut network_map: HashMap<String,Node> = HashMap::new();
    let mut orly: String = String::from("");
    let mut the_a_team: Vec<String> = Vec::new();

    for (i, line) in fs::read_to_string("input.txt")
            .unwrap().lines().enumerate() {

        if i == 0 {
            orly = String::from(line);
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

        if directions[0].chars().last().unwrap() == 'A' {
            the_a_team.push(directions[0].to_string());
        }
    }

    for team in &mut the_a_team {
        let mut multiplier: i64 = 0;
        'head: loop {
            for c in orly.chars().clone() {
                multiplier += 1;
                *team = network_map[&team.to_string()].get(c).clone();
                if team.chars().clone().last().unwrap() == 'Z' {
                    multipliers.push(multiplier);
                    break 'head;
                }                
            }
        }
    }

    let mut advent: i64 = multipliers[0];
    for i in multipliers {
        advent = lcm(advent, i);
    }

    println!("{}", advent);

}
