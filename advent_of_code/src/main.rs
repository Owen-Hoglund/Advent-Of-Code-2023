use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::env;

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;
mod day_nine;

fn main() {

    for i in 1..10{    
        let day_number = i; // Lets add user input here eventually
        match open_file(day_number){
            Ok(contents) => day_function(day_number, contents),
            Err(error) => println!("DAY {} {}", day_number, error),
        }
    }
}

fn day_function(day: u8, contents: String) {
    match day {
        1 => day_one::day_one(contents),
        2 => day_two::day_two(contents),
        3 => day_three::day_three(contents),
        4 => day_four::day_four(contents),
        5 => day_five::day_five(contents),
        6 => day_six::day_six(contents),
        7 => day_seven::day_seven(),
        8 => day_eight::day_eight(contents),
        9 => day_nine::day_nine(contents),



        _ => println!("We have not coded day {} into main()", day)
    }
}


fn open_file(file_number: u8) -> Result<String, std::io::Error>{
    let file_path = get_file_path(file_number);

    let mut file: File = File::open(file_path)?;

    let mut contents = String::new();
    
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}

fn get_file_path(file_number: u8) -> PathBuf {
    let mut path = PathBuf::new();
    path.push("src/input_files");
    path.push(file_number.to_string());
    path
}

fn split_on_lines(text_document: &str) -> Vec<String> {
    let current_operating_system = env::consts::OS;
    match current_operating_system {
        "linux" => text_document.split('\n').map(|x| x.to_string()).collect(),
        "windows" => text_document.split("\r\n").map(|x| x.to_string()).collect(),
        _ => panic!("Operating system {} is not yet supported", current_operating_system),
    }
}
