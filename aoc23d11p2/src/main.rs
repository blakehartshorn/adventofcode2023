use std::fs;

struct Galaxy {
    row: i64,
    col: i64,
}

impl Galaxy {
    fn distance(&self, neighbor: &Galaxy, empty_columns: &Vec<usize>, empty_rows: &Vec<usize>) -> i64 {

        let mut x1 = self.row;
        let mut x2 = neighbor.row;
        let mut y1 = self.col;
        let mut y2 = neighbor.col;

        for col in empty_columns {
            let mut space = vec![y1, y2, *col as i64];
            space.sort();
            if space[1] == *col as i64 {
                y1 = space[0];
                y2 = space[2] + 999999;
            }
        }

        for row in empty_rows {
            let mut space = vec![x1, x2, *row as i64];
            space.sort();
            if space[1] == *row as i64 {
                x1 = space[0];
                x2 = space[2] + 999999;
            }
        }

        return (x1 - x2).abs() + (y1 - y2).abs();

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

    // Expand rows
    let mut empty_rows: Vec<usize> = Vec::new();
    for i in 0..universe.len() {
        if universe[i].contains(&'#') {
            continue;
        }
        empty_rows.push(i);
    }
    empty_rows.reverse();

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
            advent += g.distance(galaxy, &empty_columns, &empty_rows);
        }
    }

    println!("{advent}");
}