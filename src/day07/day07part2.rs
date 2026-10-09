use std::collections::HashSet;
use std::fs;


fn solve_for_input(path: String) -> usize {
    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file {path}");
    content.lines()
        .map(|x| x.trim())
        .map(|x| x.as_bytes())
        .filter(|x| {
            let mut in_brackets = false;
            let mut seen: HashSet<(u8, u8, u8, bool)> = HashSet::new();
            for (i, c) in x.iter().enumerate() {
                if *c == b'[' || *c == b']' {
                    in_brackets = !in_brackets;
                    continue;
                }
                if i >= x.len() - 2 {
                    break;
                }
                if x[i] == x[i + 2] && x[i] != x[i + 1] {
                    if seen.contains(&(x[i + 1], x[i], x[i + 1], !in_brackets)) {
                        return true;
                    }
                    seen.insert((x[i], x[i + 1], x[i], in_brackets));
                }
            }

            false
        }).count()
}

pub fn main() {
    println!("Example input : {}", solve_for_input(String::from("./inputs/day07_example_p2.txt")));
    println!("Validation input : {}", solve_for_input(String::from("./inputs/day07_input.txt"))); // 115-116
}
