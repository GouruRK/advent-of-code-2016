use std::collections::HashMap;
use std::fs;


struct Robot {
    low: Option<u32>,
    high: Option<u32>,
    full: bool,
    action: Option<(Target, Target)>
}

#[derive(Clone, Copy)]
enum Target {
    OUTPUT,
    ROBOT(u32)
}

impl Target {
    fn from_string(target_type: &str, id: &str) -> Target {
        let id = id.parse::<u32>().unwrap();
        match target_type {
            "output" => Target::OUTPUT,
            "bot" => Target::ROBOT(id),
            _ => panic!("Unknown target: {}", target_type)
        }
    }
}


fn insert_value(robot: &mut Robot, value: u32) {
    match robot.low {
        None => {
            robot.low = Some(value);
        },
        Some(v) => {
            if v > value {
                robot.low = Some(value);
                robot.high = Some(v);
            } else {
                robot.high = Some(value);
            }
            robot.full = true;
        }
    }
}


fn solve_for_input(path: String, target1: u32, target2: u32) -> u32 {
    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file {path}");

    let mut robots: HashMap<u32, Robot> = HashMap::new();

    let mut queue = content.lines()
        .map(str::trim)
        .map(str::split_whitespace)
        .map(|x| x.collect::<Vec<_>>())
        .map(|x| {
            match x[..] {
                ["bot", id, "gives", "low", "to", target_type_1, target1, "and", "high", "to", target_type_2, target2] => {
                    let id: u32 = id.parse().unwrap();
                    let low_action = Target::from_string(target_type_1, target1);
                    let high_action = Target::from_string(target_type_2, target2);
                    let robot = robots.entry(id).or_insert(Robot { low: None, high: None, full: false, action: None });
                    robot.action = Some((low_action, high_action));
                    None
                },
                ["value", x, "goes", "to", "bot", id] => {
                    let x: u32 = x.parse().unwrap();
                    let id: u32 = id.parse().unwrap();
                    let robot = robots.entry(id).or_insert(Robot { low: None, high: None, full: false, action: None });
                    insert_value(robot, x);
                    if robot.full {
                        Some(id)
                    } else {
                        None
                    }
                },
                _ => panic!()
            }
        })
        .flat_map(|x| x)
        .collect::<Vec<_>>();


    while !queue.is_empty() {
        let source_id = queue.pop().unwrap();
        let (low, high, was_full, action) = {
            let robot = robots.get_mut(&source_id).unwrap();
            let was_full = robot.full;

            robot.full = false;
            (robot.low.unwrap(), robot.high.unwrap(), was_full, robot.action)
        };

        if !was_full { continue; }

        if low == target1 && high == target2 {
            return source_id;
        }

        match action {
            None => {},
            Some((low_action, high_action)) => {
                appy_action(&mut robots, &mut queue, low, low_action);
                appy_action(&mut robots, &mut queue, high, high_action);
            }
        }
    }

    panic!()
}

fn appy_action(robots: &mut HashMap<u32, Robot>, queue: &mut Vec<u32>, value: u32, action: Target) {
    match action {
        Target::OUTPUT => {}
        Target::ROBOT(receiver_id) => {
            let receiver = robots.entry(receiver_id).or_insert(Robot { low: None, high: None, full: false, action: None });
            insert_value(receiver, value);
            if receiver.full {
                queue.push(receiver_id);
            }
        }
    }
}

pub fn main() {
    println!("Validation input : {}", solve_for_input(String::from("./inputs/day10_example.txt"), 2, 5));
    println!("Validation input : {}", solve_for_input(String::from("./inputs/day10_input.txt"), 17, 61));
}
