use std::fs;
use std::collections::HashMap;
use phf::phf_map;

const FIVE_OF_A_KIND: i32 = 7;
const FOUR_OF_A_KIND: i32 = 6;
const FULL_HOUSE: i32 = 5;
const THREE_OF_A_KIND: i32 = 4;
const TWO_PAIR: i32 = 3;
const ONE_PAIR: i32 = 2;
const HIGH_CARD: i32 = 1;

static CARD_VAL: phf::Map<char, i32> = phf_map! {
    '1' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'T' => 10,
    'J' => 11,
    'Q' => 12,
    'K' => 13,
    'A' => 14,
};

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i32,
}

impl Hand {
    fn kind(&self) -> i32 {

        // Get card count into HashMap
        let mut card_hash: HashMap<char, i32> = HashMap::with_capacity(5);
        for c in self.cards.chars() {
            match card_hash.get(&c) {
                Some(count) => { card_hash.insert(c, count + 1); }
                None => { card_hash.insert(c, 1); }
            }
        }

        // FIVE_OF_A_KIND
        if card_hash.len() == 1 {
            return FIVE_OF_A_KIND;
        }

        // FOUR_OF_A_KIND or FULL_HOUSE
        if card_hash.len() == 2 {
            for v in card_hash.values() {
                if *v == 4 {
                    return FOUR_OF_A_KIND;
                } 
            }
            return FULL_HOUSE;
        }

        // THREE_OF_A_KIND or TWO_PAIR
        if card_hash.len() == 3 {
            if *card_hash.values().into_iter().max().unwrap() == 3 {
                return THREE_OF_A_KIND;
            } else {
                return TWO_PAIR;
            }
        }

        // ONE_PAIR
        if card_hash.len() == 4 {
            return ONE_PAIR;
        }

        return HIGH_CARD;        
    }

    // The way Rust evalulated Vectors is conveniently the way
    // the exercise tells us to sort high cards.
    fn high_card(&self) -> Vec<i32> {
        let mut high_cards: Vec<i32> = Vec::new();
        for c in self.cards.chars() {
            high_cards.push(CARD_VAL.get(&c).cloned().unwrap());
        }
        high_cards
    }

}


fn main() {
    
    let mut hands_unsorted: Vec<Hand> = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {

        if line.contains(" ") {
            let parsed_line: Vec<&str> = line.split(" ").collect();
            let hand = parsed_line[0].to_string();
            let bid: i32 = parsed_line[1].parse().unwrap();
            hands_unsorted.push(Hand {
                cards: hand,
                bid: bid,
            })
        }
    }

    let mut poker_games: Vec<Vec<Hand>> = Vec::new();
    // Create vector range since we'll be using indices
    for _ in 0..=FIVE_OF_A_KIND { poker_games.push(Vec::new()); }

    for hand in hands_unsorted.drain(..) {
        // We've previously determined ints to represent hands
        let kind = hand.kind() as usize;
        poker_games[kind].push(hand);
    }

    let mut ranked_hands: Vec<Hand> = Vec::new();
    for mut v in poker_games {
        v.sort_by(|a, b| a.high_card().cmp(&b.high_card()));
        ranked_hands.extend(v);
    }

    let mut advent_numbers: Vec<i32> = Vec::new();
    for i in 0..ranked_hands.len() {
        let rank = (1 + i) as i32;
        advent_numbers.push(ranked_hands[i].bid * rank);
    }

    let advent: i32 = advent_numbers.iter().sum();

    println!("{}", advent);

}
