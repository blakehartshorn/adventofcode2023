use std::fs;

fn main() {

    let mut advent_numbers: Vec<i32> = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parsed_numbers: String = line.chars().filter(|c| c.is_digit(10)).collect();
        if parsed_numbers.len() > 0 {
            let mut num_string = String::new();
            num_string.push(parsed_numbers.chars().next().unwrap());
            num_string.push(parsed_numbers.chars().last().unwrap());
            let num: i32 = num_string.parse().unwrap();
            advent_numbers.push(num);
        }
    }

    let advent_sum: i32 = advent_numbers.iter().sum();
    println!("{advent_sum}");
}
