#[derive(Debug, Clone, Copy)]
struct Tube {
    location: (usize, usize),
    character: char,
    nsew: (bool, bool, bool, bool),
} impl Tube {
    fn from(c: char, yx: (usize, usize)) -> Self {
        match c {
            'S' => Tube { location: yx, character: c, nsew: (true, true, true, true) },
            '|' => Tube { location: yx, character: c, nsew: (true, true, false, false) },
            '-' => Tube { location: yx, character: c, nsew: (false, false, true, true) },
            'L' => Tube { location: yx, character: c, nsew: (true, false, true, false) },
            'J' => Tube { location: yx, character: c, nsew: (true, false, false, true) },
            '7' => Tube { location: yx, character: c, nsew: (false, true, false, true) },
            'F' => Tube { location: yx, character: c, nsew: (false, true, true, false) },
            '.' => Tube { location: yx, character: c, nsew: (false, false, false, false) },
            _ => panic!("Not a valid input")
        }
    }

    // connects does not check if the pieces are adjacent, only if they theoretically match
    fn connects(self, tube: &Self) -> bool {
        if self.nsew.0 && tube.nsew.1 { true }
        else if self.nsew.1 && tube.nsew.0 { true }
        else if self.nsew.2 && tube.nsew.3 { true }
        else if self.nsew.3 && tube.nsew.2 { true }
        else{ false }
    }
}

pub fn day_ten(file_contents: String) {
    let (grid, start) = populate_grid(&file_contents);
    let circuit = get_circuit(&grid, start);
    // for c in circuit {
    //     println!("{}, {:?}", c.character, c.location);
    // }
    println!("{}", circuit.len()/2);
}

fn get_circuit(grid: &Vec<Vec<Tube>>, origin: (usize, usize)) -> Vec<Tube> {
    let mut circuit: Vec<Tube> = Vec::new();
    circuit.push(grid[origin.1][origin.0].clone());
    circuit.push(get_next(grid, origin, (usize::MAX,usize::MAX)));
    loop {
        let l = circuit.len();
        let next = get_next(grid, circuit[l - 1].location, circuit[l - 2].location);
        if next.character == 'S' {break;}
        circuit.push(next);
    }    
    circuit
}

fn get_next(grid: &Vec<Vec<Tube>>, head: (usize, usize), tail: (usize, usize)) -> Tube{
    let adjacent_elements = vec![(head.0 + 1, head.1), (head.0 - 1, head.1), (head.0, head.1 + 1), (head.0, head.1 - 1)];
    for location in adjacent_elements {
        if location == tail {continue;}

        match in_bounds(grid, location) {
            true => {
                match grid[head.1][head.0].connects(&grid[location.1][location.0]){
                    true => {return grid[location.1][location.0]},
                    false => (),
                }
            },
            false => continue,
        }
    }

    println!("{:?}", head);
    println!("{:?}", grid[head.1][head.0]);
    println!("{}", grid[0].len());
    println!("{}", grid[1].len());


    panic!("Shouldnt be possible to not find a next element");
}

fn in_bounds(grid: &Vec<Vec<Tube>>, coordinates: (usize, usize)) -> bool {
    match grid.get(coordinates.0) {
        Some(row) => match row.get(coordinates.1){
            Some(_) => true,
            None => {
                println!("Failed at {:?}", coordinates);
                false
            },
        },
        None => {
            println!("Failed at {:?}", coordinates);
            false
        },
    }
}


fn populate_grid(grid_data: &str) -> (Vec<Vec<Tube>>, (usize, usize)) {
    let mut start = (0, 0);
    let raw: Vec<Vec<char>> = grid_data.lines().map(|x| x.chars().collect()).collect();
    (raw.iter().rev().enumerate().map(|(y, line)| {
        line.iter().enumerate().map(|(x, c)| {
            if c == &'S' {start = (x, y)};
            Tube::from(*c, (x, y))
        }).collect()
    }).collect(), start)
}