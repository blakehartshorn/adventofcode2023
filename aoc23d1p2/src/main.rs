use std::fs;
//use regex::Regex;
use std::collections::HashMap;

fn main() {

    let mut english_numbers = HashMap::new();

    // There has got to be a better
    english_numbers.insert(String::from("one"), String::from("1"));
    english_numbers.insert(String::from("two"), String::from("2"));
    english_numbers.insert(String::from("three"), String::from("3"));
    english_numbers.insert(String::from("four"), String::from("4"));
    english_numbers.insert(String::from("five"), String::from("5"));
    english_numbers.insert(String::from("six"), String::from("6"));
    english_numbers.insert(String::from("seven"), String::from("7"));
    english_numbers.insert(String::from("eight"), String::from("8"));
    english_numbers.insert(String::from("nine"), String::from("9"));

    let mut advent_numbers: Vec<i32> = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {

        let mut parsed_string = String::from(line);
        /*
            After finding that the regex library for Rust can't look backward,
            I ran into issues where number words would over lap, such as 
            "twone". I'm new to rust, so there's probably a better way to do this.
            But anyway, the approach has been to simply find an index marker
            for all the words going backward and forward, put a digit in two
            separate strings, and then glue them together.

            This is awful, I know.
        */

        // Shallow, find the first word.
        let mut number_locations = HashMap::new();
        for (word, _) in &english_numbers {
            let found_number = line.find(word);
            if found_number.is_some() {
                number_locations.insert(word, found_number.unwrap());
            }
        }
        if number_locations.len() > 0 {
            let (word, number) = number_locations.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
            parsed_string.insert_str(*number, &english_numbers[*word]);
        }

        // Reverse, second word.
        number_locations = HashMap::new();
        for (word, _) in &english_numbers {
            let found_number = line.rfind(word);
            if found_number.is_some() {
                number_locations.insert(word, found_number.unwrap());
            }
        }
        if number_locations.len() > 0 {
            let (word, number) = number_locations.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
            let mut original_string = String::from(line);
            original_string.insert_str(*number, &english_numbers[*word]);
            parsed_string.push_str(&original_string);
        }

        let parsed_numbers: String = parsed_string.chars().filter(|c| c.is_digit(10)).collect();
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
