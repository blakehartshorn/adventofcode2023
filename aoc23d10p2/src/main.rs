use std::fs;

fn main() {
    
    let mut pipemap: Vec<Vec<char>> = Vec::new();
    let mut coordinates: Vec<(i32, i32)> = Vec::new();

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut travel: &str = "";
    let mut boundaries: i32 = 0;
    for (n, line) in fs::read_to_string("input.txt")
            .unwrap().lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..chars.len() {
            if chars[i] == 'S' {
                x = n;
                y = i;
                coordinates.push((n as i32,i as i32));
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
        boundaries += 1;
        let next_step = pipemap[x][y];

        // Yeah this probably needs an impl or something.
        // Enjoy this plate of spaghetti.
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
            coordinates.push((x as i32,y as i32)); // only when we change direction
            if travel == "west" {
                travel = "north";
                x -= 1;
            } else if travel == "south" {
                travel = "east";
                y += 1;
            }
        } else if next_step == 'J' {
            coordinates.push((x as i32,y as i32));
            if travel == "south" {
                travel = "west";
                y -= 1;
            } else if travel == "east" {
                travel = "north";
                x -= 1;
            }
        } else if next_step == '7' {
            coordinates.push((x as i32,y as i32));
            if travel == "east" {
                travel = "south";
                x += 1;
            } else if travel == "north" {
                travel = "west";
                y -= 1;
            }
        } else if next_step == 'F' {
            coordinates.push((x as i32,y as i32));
            if travel == "north" {
                travel = "east";
                y += 1;
            } else if travel == "west" {
                travel = "south";
                x += 1;
            }
        } else if next_step == 'S' {
            coordinates.push((x as i32,y as i32));
            break;
        }
    }

    let mut area: i32 = 0;

    // Thanks Wikipedia. Geometry class was a long time ago.
    for c in 0..coordinates.len()-1 {

        let x1 = coordinates[c].0 + 1; // in case it multiplies by zero
        let y1 = coordinates[c].1 + 1 ;
        let x2 = coordinates[c+1].0 + 1;
        let y2 = coordinates[c+1].1 + 1;
 
        area += (x2 * y1) - (x1 * y2);
    }
    area = area.abs() / 2;
    println!("{}", area - boundaries / 2 + 1);
}
