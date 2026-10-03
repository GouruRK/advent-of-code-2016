use std::fs;


fn solve_for_input(path: String) -> usize {
    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file {path}");
    content.lines()
        .map(|x| x.trim())
        .map(|x| x.split_whitespace()
            .map(|y| y.parse().unwrap()).collect()
        )
        .map(|x: Vec<i32>| (x[0], x[1], x[2]))
        .map(|(a, b, c)| {
            match (a, b, c) {
                (a, b, c) if a > b && a > c => (a, b, c),
                (a, b, c) if b > a && b > c => (b, a, c),
                _ => (c, a, b)
            }
        })
        .filter(|(a, b, c)| *b + *c > *a)
        .count()
}

pub fn main(){
    println!("Example input : {}", solve_for_input(String::from("./inputs/day03_example.txt")));
    println!("Validation input : {}", solve_for_input(String::from("./inputs/day03_input.txt")));
}
