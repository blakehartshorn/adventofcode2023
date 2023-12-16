use std::fs;
use regex::Regex;

struct Range {
    first: usize,
    last: usize,
}

fn expand_range(numbers: (i32, usize, usize)) -> Range {
    let ufirst = numbers.1.checked_sub(1);
    let ulast = numbers.2.checked_add(1);

    let first: usize;
    let last: usize;

    if ufirst.is_some() {
        first = ufirst.unwrap();
    } else {
        first = numbers.1;
    }

    if ulast.is_some() {
        last = ulast.unwrap();
    } else {
        last = numbers.2;
    }

    return Range {
        first: first,
        last: last,
    }
}

fn main() {
    let mut numbers: Vec<Vec<(i32, usize, usize)>> = Vec::new();
    let mut symbols: Vec<Vec<usize>> = Vec::new();

    let re_numbers = Regex::new(r"(\d+)").unwrap();
    let re_symbols = Regex::new(r"\*").unwrap();

    let mut advent_numbers: Vec<i32> = Vec::new();

    numbers.push(Vec::new());
    symbols.push(Vec::new());

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        numbers.push(Vec::new());
        symbols.push(Vec::new());
        let i: usize = numbers.len() - 1;
        for mat in re_numbers.find_iter(line) {
            let number: i32 = mat.as_str().parse::<i32>().unwrap();
            let mut number_range = mat.range();
            // Was having issues with single digit numbers
            let first: usize = number_range.next().unwrap();
            let last: usize = if number >= 10 {
                number_range.last().unwrap()
            } else {
                first
            };
            numbers[i].push((number, first, last));
        }
        for mat in re_symbols.find_iter(line) {
            let mut symbol_range = mat.range();
            symbols[i].push(symbol_range.next().unwrap());
        }
    }

    numbers.push(Vec::new());
    symbols.push(Vec::new());

    for i in 1..symbols.len()-1 {
        for symbol in &symbols[i] {
            let mut matches: Vec<i32> = Vec::new();
            for row in &numbers[(i-1)..=(i+1)] {
                for number in row {
                    let valid_range = expand_range(*number);
                    if valid_range.first <= *symbol && *symbol <= valid_range.last {
                        matches.push(number.0);
                    }
                }
            }
            if matches.len() == 2 {
                advent_numbers.push(matches[0] * matches[1]);
            }
        }
    }
    let advent_sum: i32 = advent_numbers.iter().sum();
    println!("{advent_sum}");
}