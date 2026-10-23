use std::collections::{HashSet, HashMap, VecDeque};
use std::fs;

enum Cell {
    Empty,
    Wall
}

enum Parity {
    Odd,
    Even
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn get_function(&self) -> fn((i32, i32)) -> (i32, i32) {
        match &self {
            Direction::Up => |(x, y)| { (x, y + 1) },
            Direction::Down => |(x, y)| { (x, y - 1) },
            Direction::Left => |(x, y)| { (x - 1, y) },
            Direction::Right => |(x, y)| { (x + 1, y) }
        }
    }
}

fn bit_parity(mut n: i32) -> Parity {
    let mut acc = 0;
    while n != 0 {
        acc += n & 1;
        n = n >> 1;
    }
    match acc % 2 {
        0 => Parity::Even,
        _ => Parity::Odd
    }
}

fn get_cell(pos: (i32, i32), magic_number: i32) -> Cell {
    let (x, y) = pos;

    let n = x*x + 3*x + 2*x*y + y + y*y;
    let n = n + magic_number;
    match bit_parity(n) {
        Parity::Odd => Cell::Wall,
        Parity::Even => Cell::Empty,
    }
}

fn get_map_value(map: &mut HashMap<(i32, i32), Cell>, pos: (i32, i32), magic_number: i32) -> &Cell {
    map.entry(pos).or_insert(get_cell(pos, magic_number))
}

fn solve_for_number(magic_number: i32, target: (i32, i32)) -> i32 {
    let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    let mut map: HashMap<(i32, i32), Cell> = HashMap::new();
    let mut treated: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: VecDeque<((i32, i32), i32)> = VecDeque::from([((1, 1), 0)]);


    while !queue.is_empty() {
        let (pos, dist) = queue.pop_front().unwrap();
        if !treated.insert(pos) {
            continue;
        }

        for direction in &directions {
            let f = direction.get_function();
            let new_pos = f(pos);

            if treated.contains(&new_pos) {
                continue;
            }

            if new_pos == target {
                return dist + 1;
            }

            match get_map_value(&mut map, new_pos, magic_number) {
                Cell::Empty => queue.push_back((new_pos, dist + 1)),
                Cell::Wall => {}
            }

        }

    }

    panic!()
}

fn solve_for_input(path: String, target: (i32, i32)) -> i32 {
    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file {path}");

    solve_for_number(content.parse::<i32>().unwrap(), target)
}



pub fn main() {
    println!("Validation input : {}", solve_for_number(10, (7, 4)));
    println!("Validation input : {}", solve_for_input(String::from("./inputs/day13_input.txt"), (31, 39)));
}
