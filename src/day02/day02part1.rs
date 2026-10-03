use std::fs;

const KEYPAD: [[char; 3]; 3] = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9']
];

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

fn solve(input: Vec<Vec<Direction>>) -> String {
    let mut x: isize = 1;
    let mut y: isize = 1;
    let mut result: Vec<char> = Vec::new();
    for line in input {
        for direction in line {
            match direction {
                Direction::Up if y - 1 != -1 => y = y - 1,
                Direction::Down if y + 1 != 3 => y = y + 1,
                Direction::Left if x - 1 != -1 => x = x - 1,
                Direction::Right if x + 1 != 3 => x = x + 1,
                _ => {}
            }
        }
        result.push(KEYPAD[y as usize][x as usize]);
    }
    result.iter().collect()
}

fn solve_for_input(path: String) -> String {
    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file {path}");
    solve(content
        .lines()
        .map(|x| x.trim().chars())
        .map(|x|
            x.map(|c|
                {
                    match c {
                        'U' => Direction::Up,
                        'D' => Direction::Down,
                        'L' => Direction::Left,
                        'R' => Direction::Right,
                        _ => panic!(),
                    }
                }
            ).collect()
        ).collect()
    )
}

pub fn main(){
    println!("Example input : {}", solve_for_input(String::from("./inputs/day02_example.txt")));
    println!("Validation input : {}", solve_for_input(String::from("./inputs/day02_input.txt")));
}
