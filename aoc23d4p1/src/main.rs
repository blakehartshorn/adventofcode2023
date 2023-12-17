use regex::Regex;
use std::fs;

fn card_string_parser(re: &Regex, card_string: &str) -> Vec::<i32> {
    let mut card: Vec<i32> = Vec::new();
    for mat in re.find_iter(card_string) {
        card.push(mat.as_str().parse::<i32>().unwrap());
    }

    return card
}

fn main() {

    let mut advent_numbers: Vec<i32> = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();

    for line in fs::read_to_string("input.txt").unwrap().lines() {

        let card_number_drop: Vec<&str> = line.split(":").collect();
        let cards: Vec<&str> = card_number_drop[1].split("|").collect();
        let left_card: Vec<i32> = card_string_parser(&re,cards[0]);
        let right_card: Vec<i32> = card_string_parser(&re, cards[1]);

        let overlap: i32 = { 
            let mut c: i32 = 0;
            for winner in left_card {
                if right_card.contains(&winner) {
                    c += 1;
                }
            }
            c
        };
        if overlap > 0 {
            let mut x: i32 = 1;
            for _ in 1..overlap {
                x += x;
            }
            advent_numbers.push(x);
        }
    }
    let advent_sum: i32 = advent_numbers.iter().sum();
    println!("{:?}", advent_sum);
}
