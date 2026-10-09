use std::fs;

fn solve(codes: Vec<&str>) -> String {
    let code_size = codes.first().unwrap().len();
    let mut result = String::new();
    let byte_codes: Vec<&[u8]> = codes.iter().map(|c| c.as_bytes()).collect();

    for i in 0..code_size {
        let mut bucket: [u32; 26] = [0; 26];
        for code in &byte_codes {
            bucket[(code[i] - b'a') as usize] += 1;
        }
        let max = bucket.iter().enumerate().filter(|x| *x.1 > 0).min_by_key(|&(_, v)| v).unwrap();
        result.push((max.0 as u8 + 97) as char);
    }

    result
}

fn solve_for_input(path: String) -> String {
    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file {path}");
    let codes = content.lines()
        .map(|x| x.trim())
        .collect::<Vec<&str>>();

    solve(codes)

}

pub fn main(){
    println!("Example input : {}", solve_for_input(String::from("./inputs/day06_example.txt")));
    println!("Validation input : {}", solve_for_input(String::from("./inputs/day06_input.txt")));
}
