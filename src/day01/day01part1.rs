use std::fs;

enum Turn {
    Left,
    Right,
}

enum Direction {
    North,
    East,
    South,
    West,
}

struct Position {
    x: i32,
    y: i32
}

impl Position {

    fn update(&self, direction: &Direction, value: i32) -> Position {
        match direction {
            Direction::North => Position { x: self.x, y: self.y + value },
            Direction::South => Position { x: self.x, y: self.y - value },
            Direction::East => Position { x: self.x - value, y: self.y },
            Direction::West => Position { x: self.x + value, y: self.y },
        }
    }

}

impl Direction {
    fn turn(self, turn: &Turn) -> Direction {
        match (self, turn) {
            (Direction::North, Turn::Left) => Direction::West,
            (Direction::North, Turn::Right) => Direction::East,

            (Direction::East, Turn::Left) => Direction::North,
            (Direction::East, Turn::Right) => Direction::South,

            (Direction::South, Turn::Left) => Direction::East,
            (Direction::South, Turn::Right) => Direction::West,

            (Direction::West, Turn::Left) => Direction::South,
            (Direction::West, Turn::Right) => Direction::North,
        }
    }
}

fn format_input(content: &str) -> Vec<(Turn, i32)> {
    content.split(',')
        .map(|x| x.trim())
        .map(|x| {
            let mut chars = x.chars();
            let dir = chars.next().unwrap();
            let steps: i32 = chars.as_str().parse().unwrap();

            match dir {
                'L' => (Turn::Left, steps),
                _   => (Turn::Right, steps)
            }
        }
    ).collect()
}

fn manhattan_distance(pos: &Position) -> i32 {
    (0 - pos.x).abs() + (0 - pos.y).abs()
}

fn solve(input: &str) -> i32 {
    let mut position = Position { x: 0, y: 0 };
    let mut direction = Direction::North;
    for (turn, step) in format_input(input) {
        direction = direction.turn(&turn);
        position = position.update(&direction, step);
    }
    manhattan_distance(&position)
}

fn solve_for_input(path: String) -> i32 {
    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file {path}");
    solve(content.trim())
}

pub fn main(){
    println!("Example input : {}", solve_for_input(String::from("./inputs/day01_example.txt")));
    println!("Validation input : {}", solve_for_input(String::from("./inputs/day01_input.txt")));
}
