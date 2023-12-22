// FINAL SOLUTION TO DAY 8

use std::collections::HashMap;
use itertools::Itertools;

pub fn day_eight(file_contents: String) {
    let map = get_map(&file_contents);
    let directions = get_directions();
    let inits = get_initial_keys(&map);
    let step_counts: Vec<i32> = inits.iter().map(|x| count_steps(&map, &directions, x)).collect();
    let result = get_lcm(step_counts);
    println!("{}", result);
}

fn count_steps(map: &HashMap<&str, (&str, &str)>, directions: &Vec<i32>, init: &str) -> i32 {
    let mut steps = 0;
    let mut val: &(&str, &str) = map.get(init).unwrap();
    let mut current: &str;
    loop {
        for i in 0..directions.len() {
            steps += 1;
            current = match directions[i] {
                0 => val.0,
                1 => val.1,
                _ => panic!("Bad Directions")
            };
            match ends_in_z(current) {
                true => return steps,
                false => val = map.get(current).unwrap(),
            }
        }
    }
}

fn get_initial_keys<'a>(map: &HashMap<&'a str, (&str, &str)>) -> Vec<&'a str> {
    map.iter().map(|x|
    {
        *x.0
    }).filter(|y| {
        y.chars().nth(2) == Some('A')
    }).collect()
}

fn ends_in_z(key: &str) -> bool {
    key.chars().nth(2) == Some('Z')
}

fn get_directions() -> Vec<i32> {
    let dir_string = "LLRLRLRRRLRLRRRLRRRLRRLLRLLRRRLRLRRLLRLRLRRLRLRLLRLRRRLRLRRLRRLRRRLRRLRRLRRLLRRLLRRRLRRLRRLRRRLRLRRLRRLLLLRLRRLRLRRLLLRRLRRRLRRRLLRRRLRRRLRRLRRRLLLRRRLLLRRLRRLRRRLRRLRRRLRRLRRRLLRLRLRRRLRRLRLRLRRRLRLRLLLRRRLRRRLRRLRRLRLRRRLRRRLLRRRLRRLRLLLRRLLRRRLRRRLRRRLLRRRLLRRLRLRRRLRRLRRRR";
    dir_string.chars().map(
        |x|
        match x {
            'L' => 0,
            'R' => 1,
            _ => panic!("error parsing L/R Direction stream")
        }
    ).collect()
}
fn get_map(file: &str) -> HashMap<&str, (&str, &str)>{
    file.lines().map(|x|
    {
        let vals: Vec<&str> = x.split_whitespace().collect();
        (vals[0], (vals[1], vals[2]))
    }).collect()
}

fn get_lcm(nums: Vec<i32>) -> i64 {
    let merged_factorizations: HashMap<i32, usize> = nums.iter().map(
        |x| 
        reduce_factorizations(prime_factorization(*x))
    ).reduce(
        |acc, x| 
        merge_factorizations(acc, x)
    ).unwrap();
    let mut result = 1;
    for (k, v) in merged_factorizations {
        result = result * i32::pow(k, v as u32) as i64;
    }
    result
}

fn merge_factorizations(fac1: HashMap<i32, usize>, fac2: HashMap<i32, usize>) -> HashMap<i32, usize>{
    let mut merged: HashMap<i32, usize> = fac1;
    for map in fac2 {
        let (key, value) = map;
        match merged.get(&key) {
            Some(value2) => {
                merged.insert(key, usize::max(value, *value2));
            },
            None => {merged.insert(key, value);},
        }
    }
    merged

}

fn reduce_factorizations(factorization: Vec<i32>) -> HashMap<i32, usize> {
    factorization.into_iter().counts()
}


fn prime_factorization(num: i32) -> Vec<i32> {
    let mut num = num;
    let mut factorization = Vec::new();
    while num != 1 {
        for i in 2..(num + 1) {
            if num % i == 0 {
                factorization.push(i);
                num = num / i;
                break;
            }
        }
    }
    factorization
}