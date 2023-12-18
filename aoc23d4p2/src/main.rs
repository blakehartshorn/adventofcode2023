use regex::Regex;
use std::fs;

// They don't recommend recursion in Rust. THIS IS GONNA BE FUN

fn card_string_parser(re: &Regex, card_string: &str) -> Vec::<i32> {
    let mut card: Vec<i32> = Vec::new();
    for mat in re.find_iter(card_string) {
        card.push(mat.as_str().parse::<i32>().unwrap());
    }

    return card
}

fn main() {

    let re = Regex::new(r"(\d+)").unwrap();
    // winners, players, score
    let mut deck_of_cards: Vec<(Vec<i32>,Vec<i32>,i32)> = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {

        //let mut score: i32 = 0;
        let card_number_drop: Vec<&str> = line.split(":").collect();
        let cards: Vec<&str> = card_number_drop[1].split("|").collect();
        let left_card: Vec<i32> = card_string_parser(&re,cards[0]);
        let right_card: Vec<i32> = card_string_parser(&re, cards[1]);

        let overlap: i32 = { 
            let mut c: i32 = 0;
            for winner in &left_card {
                if right_card.contains(&winner) {
                    c += 1;
                }
            }
            c
        };

        deck_of_cards.push((left_card, right_card, overlap));

    }

    let mut score_multiplier: Vec<i32> = vec![1; deck_of_cards.len()];
    for i in 0..deck_of_cards.len() {
        if deck_of_cards[i].2 > 0 {
            let next = i + 1;
            let score = deck_of_cards[i].2 as usize;
            for _ in 0..score_multiplier[i] {
                for x in next..(score + next) {
                    if x > deck_of_cards.len()-1 {
                        continue;
                    }
                    score_multiplier[x] += 1;
                }
            }
        }
    }

    let advent_sum: i32 = score_multiplier.iter().sum();
    println!("{:?}", advent_sum);
}
