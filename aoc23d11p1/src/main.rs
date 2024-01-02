use std::fs;

struct Galaxy {
    row: i64,
    col: i64,
}

impl Galaxy {
    fn distance(&self, neighbor: &Galaxy) -> i64 {
        let x = neighbor.row - self.row;
        let y = neighbor.col - self.col;
        return x.abs() + y.abs();
    }
}

fn main() {

    let mut universe: Vec<Vec<char>> = Vec::new();
    let mut advent: i64 = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        universe.push(line.chars().collect());
    }

    // Expand columns
    let mut empty_columns: Vec<usize> = Vec::new();
    'head: for i in 0..universe[0].len() {
        for j in 0..universe.len() {
            if universe[j][i] == '#' {
                continue 'head;
            }
        }
        empty_columns.push(i);
    }
    empty_columns.reverse();
    for col in empty_columns {
        for x in &mut universe {
            x.insert(col, '.');
        }
    }

    // Expand rows
    let mut empty_rows: Vec<usize> = Vec::new();
    for i in 0..universe.len() {
        if universe[i].contains(&'#') {
            continue;
        }
        empty_rows.push(i);
    }
    empty_rows.reverse();
    for row in empty_rows {
        universe.insert(row, vec!['.'; universe[0].len()]);
    }

    // Find galaxies
    let mut galaxies: Vec<Galaxy> = Vec::new();
    for i in 0..universe.len() {
        for j in 0..universe[0].len() {
            if universe[i][j] == '#' {
                galaxies.push(Galaxy {
                    row: i as i64,
                    col: j as i64,
                })
            }
        }
    }

    // DESTROY GALAXIES
    for _ in 0..galaxies.len() {
        let g = galaxies.pop().unwrap();
        for galaxy in &galaxies {
            advent += g.distance(galaxy);
        }
    }

    println!("{advent}");
}