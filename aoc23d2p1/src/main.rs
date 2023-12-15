use std::fs;

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

struct GameResult {
    game_id: u32,
    winner: bool,
}

fn check_cubes(game_input: String) -> GameResult {

    let (game_id, game_score_string) = {
        let x: Vec<&str> = game_input.split("Game ").collect();
        let y: Vec<&str> = x[1].split(":").collect();
        (y[0].parse::<u32>().unwrap(), y[1])
    };
    
    fn winnable(game_score_string: &str) -> bool {
        let game_score: Vec<&str> = game_score_string.split(";").collect();
        for game in game_score {
            let cube_scores: Vec<&str> = game.split(",").collect();
            for cube_score in cube_scores {
                let c: Vec<&str> = cube_score.split(" ").collect();
                let (score, color) = (c[1].parse::<u32>().unwrap(), c[2]);
                match color {
                    "red" => if score > RED_MAX { return false },
                    "green" => if score > GREEN_MAX { return false },
                    "blue" => if score > BLUE_MAX { return false },
                    _ => continue,
                };
            };
        };
        return true
    }

    let game_result = GameResult {game_id: game_id, winner: winnable(game_score_string)};
    return game_result;
}

fn main() {

    let mut advent_score: Vec<u32> = Vec::new();

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let game_result = check_cubes(line.to_string());
        if game_result.winner {
            advent_score.push(game_result.game_id);
        }
    }

    let advent_sum: u32 = advent_score.iter().sum();
    println!("{advent_sum}");

}
