use core::num;
use std::arch::x86_64;

use crate::split_on_lines;

#[derive(Debug, Clone)]
struct Game {
    id_number: u32,
    subgames: Vec<DiceColors>,
}

#[derive(Debug, Clone, Copy)]
struct DiceColors {
    red: u32,
    green: u32,
    blue: u32
}

impl DiceColors {
    pub fn from_subgame(subgame: &str) -> DiceColors {
        let mut color_counts: DiceColors = DiceColors { red: 0, green: 0, blue: 0 };
        let pairs: Vec<String> = subgame.split(",").map(|x| x.to_string()).collect();
        for pair in pairs {
            let color = Self::get_color(pair.clone());
            let count = number_extractor(pair);
            match color.as_str() {
                "red" => color_counts.red = count,
                "green" => color_counts.green = count,
                "blue" => color_counts.blue = count,
                _ => println!("Color not found"),
            }
        }
        color_counts
    }
    pub fn get_color(pair: String) -> String {
        let possible_colors = vec!("red", "green", "blue");
        for color in possible_colors {
            if pair.contains(color){
                return color.to_string()
            }
        }
        "Pair contains no known color".to_string()
    }
}

pub fn number_extractor(alphanum: String) -> u32 {
    match alphanum.chars().filter(|x| x.is_numeric()).map(|x| x.to_string()).collect::<String>().parse() {
        Ok(number) => number,
        Err(_) => todo!("ERROR HANDLING FOR UNPARSABLE COUNT"),
    }
}

// 12 red cubes, 13 green cubes, and 14 blue cubes

pub fn day_two(file_contents: String) {
    // print!("{}", file_contents);
    let mut games: Vec<Game> = Vec::new();
    let game_records = split_on_lines(file_contents);
    for record in game_records {
        games.push(game_from_record(record));
    }
    let power_sum: u32 = games.iter().map(|x| get_power(x.clone())).sum();
    println!("SUM: {}", power_sum);
    // let bag = DiceColors { red: 12, green: 13, blue: 14 };
    // let x = games.iter().filter(|x| is_game_possible(x, &bag)).fold(0, |acc, x| acc + x.id_number);
    // println!("{}", x);
    // // dbg!(games);
}

fn game_from_record(record: String) -> Game {
    let id = get_id_from_record(&record);
    let subgames = get_subgames_from_record(&record);
    return Game {id_number: id, subgames: subgames};
}

fn get_id_from_record(record: &String) -> u32 {
    let id_section = record.split(":").next();
    match id_section {
        Some(game_id) => 
            match game_id.chars().filter(|x| x.is_numeric()).map(|x| x.to_string()).collect::<String>().parse(){
                Ok(id_number) => id_number,
                Err(_) => todo!(),
            }
        None => todo!(),
    }
}

fn get_subgames_from_record(record: &String) -> Vec<DiceColors> {
    let subgames_section = &record.split(":").map(|x| x.to_string()).collect::<Vec<String>>()[1];
    let raw_subgames = subgames_section.split(";").map(|x| x.to_string()).collect::<Vec<String>>();
    let mut subgames: Vec<DiceColors> = Vec::new();
    for raw_game in raw_subgames {
        subgames.push(DiceColors::from_subgame(&raw_game));
    }
    subgames
}


fn is_game_possible(game: &Game, bag: &DiceColors) -> bool {
    for subgame in game.subgames.clone() {
        if !is_subgame_possible(subgame, &bag) {return false}
    }
    return true;
}

fn is_subgame_possible(subgame: DiceColors, bag: &DiceColors) -> bool{
    if subgame.red > bag.red {false}
    else if subgame.blue > bag.blue {false}
    else if subgame.green > bag.green {false}
    else {true}
}

fn get_power(game: Game) -> u32{
    dbg!(game.clone());
    let mut min: DiceColors = DiceColors { red: 0, green: 0, blue: 0 };
    for subgame in game.subgames {
        if subgame.red > min.red {min.red = subgame.red} else {min.red = min.red};
        if subgame.green > min.green {min.green = subgame.green} else {min.green = min.green};
        if subgame.blue > min.blue {min.blue = subgame.blue} else {min.blue = min.blue};
    }
    dbg!(min);
    return min.red * min.green * min.blue
}