use std::fs;

const KEYPAD: [[char; 5]; 5] = [
    ['\0', '\0', '1', '\0', '\0'],
    ['\0', '2',  '3', '4',  '\0'],
    ['5',  '6',  '7', '8',  '9'],
    ['\0', 'A',  'B', 'C',  '\0'],
    ['\0', '\0', 'D', '\0', '\0'],
];

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

fn solve(input: Vec<Vec<Direction>>) -> String {
    let mut x: isize = 0;
    let mut y: isize = 2;
    let mut result: Vec<char> = Vec::new();
    for line in input {
        for direction in line {
            let (ny, nx) = match direction {
                Direction::Up => (y - 1, x),
                Direction::Down => (y + 1, x),
                Direction::Left => (y, x - 1),
                Direction::Right => (y, x + 1),
            };
            if KEYPAD.get(ny as usize).and_then(|r| r.get(nx as usize)).unwrap_or(&'\0') != &'\0' {
                y = ny;
                x = nx;
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
