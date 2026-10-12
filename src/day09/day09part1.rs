use std::fs;

fn solve_for_iterator<I>(mut iterator: I) -> usize
where
    I: Iterator<Item = char>,
{
    let mut total_length : usize = 0;

    loop {
        match iterator.next() {
            None => break,
            Some(c) => {
                match c {
                    '(' => {
                        let pattern_size = read_number(&mut iterator);
                        let duplication_size = read_number(&mut iterator);
                        total_length += pattern_size * duplication_size;
                        (0..pattern_size).for_each(|_| { iterator.next(); });
                    }
                    _ => total_length += 1
                }
            }
        }
    }

    total_length
}

fn read_number<I>(iterator: &mut I) -> usize
where
    I: Iterator<Item = char>,
{
    let mut n: usize = 0;
    loop {
        match iterator.next() {
            None => panic!(),
            Some(c) if c.is_digit(10) => {
                n = n*10 + c.to_digit(10).unwrap() as usize;
            }
            _ => break,
        }
    }
    n
}

fn solve_for_input(path: String) -> usize {
    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file {path}");

    let iterator = content.lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .flat_map(str::chars);

    solve_for_iterator(iterator)
}

pub fn main(){
    println!("Example input 'ADVENT' : {}", solve_for_iterator(String::from("ADVENT").chars()));
    println!("Example input 'A(1x5)BC' : {}", solve_for_iterator(String::from("A(1x5)BC").chars()));
    println!("Example input '(3x3)XYZ' : {}", solve_for_iterator(String::from("(3x3)XYZ").chars()));
    println!("Example input 'A(2x2)BCD(2x2)EFG' : {}", solve_for_iterator(String::from("A(2x2)BCD(2x2)EFG").chars()));
    println!("Example input '(6x1)(1x3)A' : {}", solve_for_iterator(String::from("(6x1)(1x3)A").chars()));
    println!("Example input 'X(8x2)(3x3)ABCY' : {}", solve_for_iterator(String::from("X(8x2)(3x3)ABCY").chars()));

    println!("Validation input : {}", solve_for_input(String::from("./inputs/day09_input.txt")));
}
