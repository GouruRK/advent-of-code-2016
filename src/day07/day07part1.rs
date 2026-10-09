use std::fs;


fn solve_for_input(path: String) -> usize {
    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file {path}");
    content.lines()
        .map(|x| x.trim())
        .map(|x| x.as_bytes())
        .filter(|x| {
            let mut in_brackets = false;
            let mut motif_found = false;
            for (i, c) in x.iter().enumerate() {
                if *c == b'[' || *c == b']' {
                    in_brackets = !in_brackets;
                    continue;
                }
                if i >= x.len() - 3 {
                    break;
                }
                if x[i] == x[i + 3] && x[i + 1] == x[i + 2] && x[i] != x[i + 1] {
                    if in_brackets {
                        return false;
                    }
                    motif_found = true;
                }
            }

            motif_found
        }).count()
}

pub fn main(){
    println!("Example input : {}", solve_for_input(String::from("./inputs/day07_example.txt")));
    println!("Validation input : {}", solve_for_input(String::from("./inputs/day07_input.txt"))); // 115-116
}
