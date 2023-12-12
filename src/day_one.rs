pub fn day_one(file_contents: String) {

    let converted_contents = digitize_spelled_numbers(file_contents);
    let lines:Vec<String> = converted_contents.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
    let filtered_contents = remove_letters(lines);
    let outer_bounds: Vec<String> = get_outer_bounds(filtered_contents);
    let digitized_vector: Vec<u32> = number_parser(outer_bounds);
    let final_sum: u32 = digitized_vector.iter().sum();
    println!("original value: {},  ", final_sum);
}

fn number_parser(lines: Vec<String>) -> Vec<u32> {
    lines.iter().map(|x| x.parse()).flat_map(|x| x).collect()
}

fn get_outer_bounds(lines: Vec<String>) -> Vec<String> {
    lines.iter().map(|x| get_first_last(x)).collect()
}

fn get_first_last(line: &String) -> String{
    let mut new_line: String = "".to_string();
    new_line.push(line.chars().clone().next().unwrap());
    new_line.push(line.chars().clone().last().unwrap());
    println!("original: {}, modified: {}", line, new_line);
    new_line
}

fn remove_letters(lines: Vec<String>) -> Vec<String> {
    lines.iter().map(
        |line|
        line.chars().filter(|x| x.is_numeric()).collect()
    ).collect()
}


fn digitize_spelled_numbers(file_contents: String) -> String{
    let digits = vec!
    [
        "oneight",
        "twone",
        "threeight",
        "fiveight",
        "sevenine",
        "eightwo",
        "eighthree",
        "nineight",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];
    let mut modified_contents = file_contents.clone();
    for digit in digits {
        modified_contents = modified_contents.replace(digit, digit_matching(digit));
    }
    // println!("{}", modified_contents);
    modified_contents

}

fn digit_matching(spelled_digit: &str) -> &str {
    match  spelled_digit {
        "oneight" => "18",
        "twone" => "21",
        "threeight" => "38",
        "fiveight" => "58",
        "sevenine" => "79",
        "eightwo" => "82",
        "eighthree" => "83",
        "nineight" => "98",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => panic!("Not a digit")
    }
}