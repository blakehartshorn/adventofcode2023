use std::fs;
use std::collections::HashMap;

fn check_cubes(game_input: String) -> u32 {

    let game_score_string = {
        let x: Vec<&str> = game_input.split(":").collect();
        x[1]
    };
    
    let mut maximums: HashMap<&str, Vec<u32>> = HashMap::new();
    maximums.insert("red", Vec::new());
    maximums.insert("green",Vec::new());
    maximums.insert("blue", Vec::new());
    let game_score: Vec<&str> = game_score_string.split(";").collect();
    for game in game_score {
        let cube_scores: Vec<&str> = game.split(",").collect();
        for cube_score in cube_scores {
            let c: Vec<&str> = cube_score.split(" ").collect();
            let (score, color) = (c[1].parse::<u32>().unwrap(), c[2]);
            maximums.get_mut(color).unwrap().push(score);
        };
    };
    let red_max = maximums["red"].iter().max().unwrap();
    let green_max = maximums["green"].iter().max().unwrap();
    let blue_max = maximums["blue"].iter().max().unwrap();

    return red_max * green_max * blue_max;
}

fn main() {

    let mut advent_score: Vec<u32> = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let count_cubes = check_cubes(line.to_string());
        advent_score.push(count_cubes);
        println!("{count_cubes}");
    }

    let advent_sum: u32 = advent_score.iter().sum();
    println!("{advent_sum}");
}
