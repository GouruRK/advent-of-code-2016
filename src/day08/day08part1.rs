use std::fs;

const INPUT_WIDTH: usize = 50;
const INPUT_HEIGHT: usize = 6;

const EXAMPLE_WIDTH: usize = 7;
const EXAMPLE_HEIGHT: usize = 3;

enum GridElement {
    ROW,
    COLUMN
}

enum Command {
    Rect(usize, usize),
    Rotate(GridElement, usize, usize)
}

impl GridElement {
    fn from_string(s: &str) -> GridElement {
        match s {
            "row" => GridElement::ROW,
            "column" => GridElement::COLUMN,
            _ => panic!()
        }
    }
}



impl Command {
    fn from_string(s: &str) -> Command {
        let tokens = s.split_whitespace().collect::<Vec<_>>();
        match tokens[..] {
            ["rect", size_data] => {
                let mut sizes = size_data.split("x");
                Command::Rect(sizes.next().unwrap().parse().unwrap(), sizes.next().unwrap().parse().unwrap())
            },
            ["rotate", element, index, "by", offset] => {
                let index: usize = index.split("=").last().unwrap().parse().unwrap();
                Command::Rotate(GridElement::from_string(element), index, offset.parse::<usize>().unwrap())
            }
            _ => panic!()
        }
    }

    fn execute_command<const W: usize, const H: usize>(self, grid: &mut [[u32; W]; H]) {
        match self {
            Command::Rect(width, height) => {
                (0..height).for_each(|y| {
                    (0..width).for_each(|x| {
                        grid[y][x] = 1;
                    })
                })
            },
            Command::Rotate(side, index, offset) => {
                match side {
                    GridElement::ROW => grid[index].rotate_right(offset),
                    GridElement::COLUMN => {
                        let mut array = [0; H];

                        for i in 0..H {
                            array[i] = grid[i][index];
                        }

                        array.rotate_right(offset);

                        for i in 0..H {
                            grid[i][index] = array[i];
                        }

                    }
                }
            }
        }
    }
}


fn solve_for_input<const W: usize, const H: usize>(path: String, mut grid: [[u32; W]; H]) -> u32 {
    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file {path}");

    content.lines()
        .map(|x| x.trim())
        .map(|x| Command::from_string(x))
        .for_each(|x| x.execute_command(&mut grid));

    grid.iter().flat_map(|x| x.iter()).sum()
}

pub fn main(){
    println!("Example input : {}", solve_for_input(String::from("./inputs/day08_example.txt"), [[0; EXAMPLE_WIDTH]; EXAMPLE_HEIGHT]));
    println!("Validation input : {}", solve_for_input(String::from("./inputs/day08_input.txt"), [[0; INPUT_WIDTH]; INPUT_HEIGHT]));
}
