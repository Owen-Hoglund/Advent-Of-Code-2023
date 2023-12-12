use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;

mod day_one;
mod day_two;
mod day_three;
mod day_four;


fn main() {
    let day_number = 4; // Lets add user input here eventually

    match open_file(day_number){
        Ok(contents) => day_function(day_number, contents),
        Err(error) => println!("{}", error),
    }
}

fn day_function(day: u8, contents: String) {
    match day {
        1 => day_one::day_one(contents),
        2 => day_two::day_two(contents),
        3 => day_three::day_three(contents),
        4 => day_four::day_four(contents),


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

fn split_on_lines(text_document: String) -> Vec<String> {
    text_document.split("\n").map(|x| x.to_string()).collect()
}

fn split_on_space(line: &String) -> Vec<String> {
    line.split(" ").map(|element| element.to_string()).filter(|x| !x.is_empty()).collect()
}


// use only if you are CONFIDENT that the String contains a numeral or this will likely brick the program. 
fn number_extractor(text: String) -> i32 {
    text.chars().filter(|x| x.is_numeric()).map(|x| x.to_string()).collect::<String>().parse().unwrap()
}