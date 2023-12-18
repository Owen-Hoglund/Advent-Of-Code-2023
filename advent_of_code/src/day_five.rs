// FINAL SOLUTION TO DAY FIVE
use core::panic;
use std::ops::Range;
use std::cmp;

struct Mapping {
    source_category: Category,
    destination_category: Category,
    source_range: Range<u64>,
    additive: i64
}
#[derive(Clone)]
struct Element {
    category: Category,
    range: Range<u64>
}

#[derive(Clone, PartialEq)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
} 
impl Category {
    fn next(self) -> Category{
        match self {
            Category::Seed => Category::Soil,
            Category::Soil => Category::Fertilizer,
            Category::Fertilizer => Category::Water,
            Category::Water => Category::Light,
            Category::Light => Category::Temperature,
            Category::Temperature => Category::Humidity,
            Category::Humidity => Category::Location,
            Category::Location => panic!("Attempting to map element "),
        }
    }
    pub fn get_by_str(s: &str) -> Category {
        match s {
            "seed" => Category::Seed,
            "soil" => Category::Soil,
            "fertilizer" => Category::Fertilizer,
            "water" => Category::Water,
            "light" => Category::Light,
            "temperature" => Category::Temperature,
            "humidity" => Category::Humidity,
            "location" => Category::Location,
            _ => panic!("failed to convert &str {} to Category", s)
        }
    }
}


pub fn day_five(file: String) {
    let (elements, map) = get_contents(&file);
    let final_elements = map_all_elements(&elements, &map);
    let mut min = u64::MAX;
    for e in final_elements {
        if e.range.start <= min {min = e.range.start}
    }
    println!("{}", min)
}

fn map_all_elements(elements: &Vec<Element>, map: &Vec<Mapping>) -> Vec<Element> {
    let batch_type: Category = elements[0].category.clone();
    if batch_type == Category::Location {return elements.to_vec();}
    else {
        let mut new_elements: Vec<Element> = Vec::new();
        for element in elements {
            new_elements.append(&mut map_element(element, map));
        }
        return map_all_elements(&new_elements, map);
    }
}

fn map_element(element: &Element, map: &Vec<Mapping>) -> Vec<Element>{
    let mut range = element.range.clone();
    let mut next_elements: Vec<Element> = Vec::new();
    let source = element.category.clone();

    let maps = map.iter().filter(|x| x.source_category == source);
    for map in maps {
        match range_overlap(&map.source_range, &range) {
            Some(overlap) => {
                let range_in_destination = (overlap.start as i64 + map.additive) as u64..(overlap.end as i64+map.additive) as u64;
                next_elements.push(
                    Element { category: map.destination_category.clone(), range: range_in_destination }
                );

                let bottom_range = range.start..map.source_range.start;
                match bottom_range.is_empty() {
                    true => (),
                    false => next_elements.push(
                        Element { category: map.destination_category.clone(), range: bottom_range }
                    ),
                }
                range.start = map.source_range.end;
            },
            None => (),
        }
    }
    match range.is_empty() {
        true => (),
        false => next_elements.push(
            Element { category: Category::next(source), range: range }
        )
    }

    next_elements
}

fn range_overlap(r1: &Range<u64>, r2: &Range<u64>) -> Option<Range<u64>> {
    let start = cmp::max(r1.start, r2.start);
    let end = cmp::min(r1.end, r2.end);

    match (start..end).is_empty() {
        true => None,
        false => Some(start..end),
    } 
}

fn get_contents(file_contents: &str) -> (Vec<Element>, Vec<Mapping>) {
    fn get_elements() -> Vec<Element> {
        SEEDS.iter().map(
            |x|
            Element {
                category: Category::Seed,
                range: x.0..(x.0+x.1),
            }
        ).collect::<Vec<Element>>()
    }

    fn get_maps(file_contents: &str) -> Vec<Mapping> {
        let mut maps: Vec<Mapping> = Vec::new();
        for section in file_contents.split("\n\n") {
            let mut lines = section.split("\n");
            let (source_category, destination_category) = get_map_categories(lines.next().unwrap());
            while let Some(mapping) = lines.next() {
                maps.push(parse_mapping(mapping, source_category.clone(), destination_category.clone()));
            }
        }
        maps.sort_by_key(|x| x.source_range.start);
        maps
    }

    fn parse_mapping(mapping: &str, source: Category, destination: Category) -> Mapping {
        let nums: Vec<u64> = mapping.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
        let (destination_range_start, source_range_start, range_length) = (nums[0], nums[1], nums[2]);
        Mapping { 
            source_category: source, 
            destination_category: destination, 
            source_range: source_range_start..source_range_start + range_length, 
            additive: destination_range_start as i64 - source_range_start as i64,
        }
    }

    fn get_map_categories(s: &str) -> (Category, Category) {
        let mut cats = s.split("-to-").map(|x| Category::get_by_str(x));
        (cats.next().unwrap(), cats.next().unwrap())
    }
    (get_elements(), get_maps(file_contents))
}

static SEEDS: &'static [(u64, u64)] = &[
    (5844012, 110899473),
    (1132285750, 58870036),
    (986162929, 109080640),
    (3089574276, 100113624),
    (2693179996, 275745330),
    (2090752257, 201704169),
    (502075018, 396653347),
    (1540050181, 277513792),
    (1921754120, 26668991),
    (3836386950, 66795009)
];

// static SEEDS: &'static [(u64, u64)] = &[(79, 14), (55, 13)];
