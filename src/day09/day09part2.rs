use std::fs;


fn solve_for_iterator(iterator: &mut dyn Iterator<Item=char>) -> u64
{
    let mut total_length : u64 = 0;

    loop {
        match iterator.next() {
            None => break,
            Some(c) => {
                match c {
                    '(' => {
                        let pattern_size = read_number(iterator);
                        let duplication_size = read_number(iterator);

                        let mut sub_pattern = (0..pattern_size).map(|_| iterator.next().unwrap());
                        let sub_pattern_size = solve_for_iterator(&mut sub_pattern);

                        total_length += sub_pattern_size*duplication_size;
                    }
                    _ => total_length += 1
                }
            }
        }
    }
    total_length
}

fn read_number(iterator: &mut dyn Iterator<Item = char>) -> u64 {
    let mut n: u64 = 0;
    loop {
        match iterator.next() {
            None => panic!(),
            Some(c) if c.is_digit(10) => {
                n = n*10 + c.to_digit(10).unwrap() as u64;
            }
            _ => break,
        }
    }
    n
}

fn solve_for_string(string: String) -> u64 {
    let mut iterator = string.chars();
    solve_for_iterator(&mut iterator)
}

fn solve_for_input(path: String) -> u64 {
    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file {path}");

    let mut iterator = content.lines()
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .flat_map(str::chars);

    solve_for_iterator(&mut iterator)
}

pub fn main(){
    println!("Example input '(27x12)(20x12)(13x14)(7x10)(1x12)A' : {}", solve_for_string(String::from("(27x12)(20x12)(13x14)(7x10)(1x12)A")));
    println!("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN' : {}", solve_for_string(String::from("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")));

    println!("Validation input : {}", solve_for_input(String::from("./inputs/day09_input.txt")));
}
