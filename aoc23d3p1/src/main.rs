use std::fs;
use regex::Regex;

/*
    Today's assignment will be brought to you by egregious looping
    and not bothering to buffer the input file.

    Don't try this at home, kids! Well, in production at least.
*/

struct Range {
    first: usize,
    last: usize,
}

// Still figuring out how to deal with this whole usize thing.
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

    // row (number, left, right)
    let mut numbers: Vec<Vec<(i32, usize, usize)>> = Vec::new();
    let mut symbols: Vec<Vec<usize>> = Vec::new();

    let re_numbers = Regex::new(r"(\d+)").unwrap();
    let re_symbols = Regex::new(r"[^0-9|\.]").unwrap();

    let mut advent_numbers: Vec<i32> = Vec::new();

    // Staring with empty rows so we can scroll back. Not the most elegant solution.
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

    // Appending empty rows. Hey, this one is tedious and I've been doing this all day.
    numbers.push(Vec::new());
    symbols.push(Vec::new());

    for i in 1..numbers.len()-1 {
        'head: for x in &numbers[i] {
            let valid_range = expand_range(*x);
            for row in &symbols[(i-1)..=(i+1)] {
                for symbol in row {
                    // error: comparison operators cannot be chained BOOOO
                    if valid_range.first <= *symbol && *symbol <= valid_range.last {
                        advent_numbers.push(x.0);
                        continue 'head;
                    }
                }
            }
        }
    }
    let advent_sum: i32 = advent_numbers.iter().sum();
    println!("{advent_sum}");
}
