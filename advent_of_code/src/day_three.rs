// FINAL SOLUTION TO DAY THREE

use core::panic;
use std::cmp;


#[derive(Clone, Debug, PartialEq)]
struct PartNumber {
    y_x1_x2: (usize, usize, usize),
    number: u32,
}

#[derive(Clone)]
struct Gear {
    coordinates_y_x: (usize, usize),
    // adjacent_parts: Vec<PartNumber>,
    gear_ratio: u32,
}
pub fn day_three(file_content: String) {
    let engine_schematic = load_schematic_to_vector(file_content);
    let parts: Vec<PartNumber> = find_numbers(engine_schematic.clone());
    let _valid_parts = part_validation(engine_schematic.clone(), parts.clone());
    let unfinished_gears = find_gears(&engine_schematic.clone());
    let gears = gear_finisher(unfinished_gears, &engine_schematic, parts);
    println!("{}", gears.iter().map(|x| x.gear_ratio).sum::<u32>());
    
}

fn gear_finisher(gears: Vec<Gear>, schematic: &Vec<Vec<char>>, parts: Vec<PartNumber>) -> Vec<Gear>{
    let mut finished_gears: Vec<Gear> = Vec::new();
    for gear in gears {
        let mut adjacent_parts: Vec<PartNumber> = Vec::new();
        for i in gear.coordinates_y_x.0 - 1..=gear.coordinates_y_x.0 + 1 {
            for j in gear.coordinates_y_x.1 - 1..=gear.coordinates_y_x.1 + 1 {
                match schematic.get(i) {
                    Some(row) => {
                        match row.get(j) {
                            Some(c) => if c.is_numeric() {
                                adjacent_parts.push(num_at_coordinates((i,j), &parts));
                            },
                            None => (),
                        }
                    },
                    None => (),
                }
            }
        }
        let adjacent_parts = deduplicate_parts(&adjacent_parts);
        match adjacent_parts.len() {
            2 => finished_gears.push(
                Gear {
                    coordinates_y_x: gear.coordinates_y_x, 
                    // adjacent_parts: adjacent_parts.clone(), 
                    gear_ratio: get_gear_ratio(&adjacent_parts)
                }),
            _ => (),
        }
    }
    finished_gears
}

fn deduplicate_parts(parts: &Vec<PartNumber>) -> Vec<PartNumber> {
    let mut unique_parts = parts.clone();
    unique_parts.dedup();
    unique_parts
}

fn get_gear_ratio(parts: &Vec<PartNumber>) -> u32 {
    parts.iter().map(|x| x.number).product::<u32>()
}

fn num_at_coordinates(coordinates: (usize, usize), parts: &Vec<PartNumber>) -> PartNumber {
    for part in parts {
        if part.y_x1_x2.0 == coordinates.0 && (part.y_x1_x2.1 <= coordinates.1) && (coordinates.1 <= part.y_x1_x2.2) {
            return part.clone();
        }
    }
    panic!("No PartNumber found at coordinates");
}

fn find_gears(schematic: &Vec<Vec<char>>) -> Vec<Gear> {
    let mut gears: Vec<Gear> = Vec::new();
    for (y, row) in schematic.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'*' {
                gears.push(
                    Gear { 
                        coordinates_y_x: (y, x), 
                        // adjacent_parts: Vec::new(),
                        gear_ratio:  0})
            }
        }
    }
    gears
}

fn part_validation(schematic: Vec<Vec<char>>, parts: Vec<PartNumber>) -> Vec<PartNumber> {
    let mut valid_parts: Vec<PartNumber> = Vec::new(); 
    for part in parts  {
        let border: Vec<char> = get_border(&schematic, part.y_x1_x2, 1);
        if check_border(border) {valid_parts.push(part);}
    }

    valid_parts
}

fn check_border(border: Vec<char>) -> bool{
    border.iter().filter(|x| !x.is_alphanumeric() && x != &&'.').count() > 0
}

fn get_border(schematic: &Vec<Vec<char>>, coordinates: (usize, usize, usize), border_width: u32,) -> Vec<char> {
    let mut border:Vec<char> = Vec::new();
    let y1 = cmp::max(0, coordinates.0 as i32 - border_width as i32) as usize;
    let y2 = coordinates.0 + border_width as usize;
    let x1 = cmp::max(0, coordinates.1 as i32 - border_width as i32) as usize;
    let x2 = coordinates.2 + border_width as usize;
    for i in y1..=y2 {
        for j in x1..=x2 {
            match schematic.get(i) {
                Some(row) => {
                    match row.get(j) {
                        Some(c) => border.push(*c),
                        None => border.push('.'),
                    }
                },
                None => border.push('.'),
            }
        }
    }
    border
}



fn load_schematic_to_vector(schematic_string: String) -> Vec<Vec<char>> {
    let mut schematic: Vec<Vec<char>> = Vec::new();
    for row in schematic_string.split('\n') {
        schematic.push(row.chars().collect::<Vec<char>>());
    }
    schematic
}

fn find_numbers(schematic: Vec<Vec<char>>) -> Vec<PartNumber> {
    let mut parts: Vec<PartNumber> = Vec::new();
    for (y, row) in schematic.iter().enumerate() {
        parts.append(&mut numbers_in_row(row.to_vec(), y));
    }
    parts
}

fn numbers_in_row(row: Vec<char>, y: usize) -> Vec<PartNumber> {
    let mut parts: Vec<PartNumber> = Vec::new();
    let mut digit_buffer: String = "".to_string();
    let mut buffer_is_open = false;
    let mut coordinates: (usize, usize, usize) = (y, 0_usize, 0_usize);

    for (x, c) in row.iter().enumerate() {
        if c.is_numeric() {
            
            digit_buffer.push(*c);

            if !buffer_is_open {
                buffer_is_open = true;
                coordinates = (y, x, x);
            } else {
                coordinates.2 = x;
            }

        } else if buffer_is_open {
            buffer_is_open = false;
            parts.push(PartNumber { y_x1_x2: coordinates, number: digit_buffer.parse().unwrap()});
            digit_buffer = "".to_string();
        }
    }
    if buffer_is_open {
        parts.push(PartNumber { y_x1_x2: coordinates, number: digit_buffer.parse().unwrap()});
    }

    parts
}