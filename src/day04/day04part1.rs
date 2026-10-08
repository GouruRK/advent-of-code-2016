use std::fs;

struct Room {
    encryption: String,
    sector: u32,
    checksum: String
}

impl Room {
    fn from_string(input: &str) -> Room {
        let mut chars = input.chars().rev();
        let checksum = (&mut chars).skip(1).take_while(|ch| *ch != '[').collect::<String>().chars().rev().collect::<String>();
        let sector = (&mut chars).take_while(|ch| *ch != '-').collect::<String>().chars().rev().collect::<String>().parse::<u32>().unwrap();
        let encryption = (&mut chars).collect::<String>().split('-').collect::<String>();
        Room { encryption, sector, checksum }
    }

    fn is_valid(&self) -> bool {
        let a_code = 'a' as usize;
        let mut buckets: [u32; 26] = [0; 26];

        for ch in self.encryption.chars() {
            buckets[(ch as usize) - a_code] += 1;
        }

        let mut freq_list: Vec<(char, u32)> = buckets
            .iter()
            .enumerate()
            .filter(|&(_, &count)| count > 0)
            .map(|(i, &count)| ((i + a_code) as u8 as char, count))
            .collect();

        freq_list.sort_by(|a, b| {
            b.1.cmp(&a.1)
                .then_with(|| a.0.cmp(&b.0))
        });

        freq_list.iter().take(5).map(|x| x.0).collect::<String>() == self.checksum
    }
}

fn solve_for_input(path: String) -> usize {
    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file {path}");
    content.lines()
        .map(|x| x.trim())
        .map(Room::from_string)
        .filter(Room::is_valid)
        .map(|room| room.sector as usize)
        .sum()
}

pub fn main(){
    println!("Example input : {}", solve_for_input(String::from("./inputs/day04_example.txt")));
    println!("Validation input : {}", solve_for_input(String::from("./inputs/day04_input.txt")));
}
