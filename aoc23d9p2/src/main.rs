use std::fs;

fn next_line(p: Vec<i64>) -> Vec<i64> {
    let mut n: Vec<i64> = Vec::new();
    for i in 0..(p.len()-1) {
        n.push(p[i+1] - p[i]);
    }
    n
}

fn next_number(p: Vec<i64>) -> i64 {
    let mut rows: Vec<Vec<i64>> = Vec::new();
    let mut n: Vec<i64> = p;
    loop {
        rows.push(n.clone());
        n = next_line(n);

        if n.iter().find(|&&z| z != 0).is_none() {
            rows.push(n.clone());
            break;
        }
    }

    let mut x: i64 = 0;
    for i in (0..(rows.len())).rev() {
        if i == 0 {
            break;
        }
        x = rows[i-1].first().unwrap() - rows[i].first().unwrap();
        let mut new_row = vec![x];
        new_row.append(&mut rows[i-1]);
        rows[i-1] = new_row;
    }
    x
}

fn main() {

    let mut advent: i64 = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        if !line.contains(" ") { continue }
        let num_strings: Vec<&str>;
        num_strings = line.split(" ").collect();
        let mut p: Vec<i64> = Vec::new();
        for num in num_strings {
            p.push(num.parse().unwrap());
        }
        advent += next_number(p);
    }

    println!("{}", advent);
}
