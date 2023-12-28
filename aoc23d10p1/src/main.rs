use std::fs;

fn main() {
    
    let mut pipemap: Vec<Vec<char>> = Vec::new();

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut travel: &str = "";
    let mut counter: i64 = 0;
    for (n, line) in fs::read_to_string("input.txt")
            .unwrap().lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..chars.len() {
            if chars[i] == 'S' {
                x = n;
                y = i;
            }
        }
        pipemap.push(chars);
    }

    // Find path from start.
    if (y + 1) <= pipemap[x].len() 
        && String::from("-J7").contains(pipemap[x][y+1]) {
        travel = "east";
        y += 1;
    } else if (y as i32 - 1) >= 0 
        && String::from("-LF").contains(pipemap[x][y-1]) {
        travel = "west";
        y -= 1;
    } else if (x as i32 - 1) >= 0 
        && String::from("|7F").contains(pipemap[x-1][y]) {
        travel = "north";
        x -= 1;
    } else if (x + 1) <= pipemap.len() 
        && String::from("|LJ").contains(pipemap[x+1][y]) {
        travel = "south";
        x += 1;
    }

    loop {
        counter += 1;

        let next_step = pipemap[x][y];
        //println!("{}:{} - {} - {} - {}", x, y, counter, travel, next_step);

        // You could say my solution is a little... iffy.
        if next_step == '|' {
            if travel == "south" {
                x += 1;
            } else if travel == "north" {
                x -= 1;
            }
        } else if next_step == '-' {
            if travel == "west" {
                y -= 1;
            } else if travel == "east" {
                y += 1;
            }
        } else if next_step == 'L' {
            if travel == "west" {
                travel = "north";
                x -= 1;
            } else if travel == "south" {
                travel = "east";
                y += 1;
            }
        } else if next_step == 'J' {
            if travel == "south" {
                travel = "west";
                y -= 1;
            } else if travel == "east" {
                travel = "north";
                x -= 1;
            }
        } else if next_step == '7' {
            if travel == "east" {
                travel = "south";
                x += 1;
            } else if travel == "north" {
                travel = "west";
                y -= 1;
            }
        } else if next_step == 'F' {
            if travel == "north" {
                travel = "east";
                y += 1;
            } else if travel == "west" {
                travel = "south";
                x += 1;
            }
        } else if next_step == 'S' {
            break;
        }
    }

    println!("{}", counter / 2);
}
