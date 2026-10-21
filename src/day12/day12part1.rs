use std::collections::HashMap;
use std::fs;

enum ValueType {
    Num(i32),
    Register(i32)
}

impl ValueType {
    fn from_string(s: &str) -> ValueType {
        match s {
            "a" => ValueType::Register(0),
            "b" => ValueType::Register(1),
            "c" => ValueType::Register(2),
            "d" => ValueType::Register(3),
            _ => ValueType::Num(s.parse::<i32>().unwrap()),
        }
    }
}

enum Instruction {
    Cpy(ValueType, usize),
    Inc(usize),
    Dec(usize),
    Jnz(ValueType, i32),
}

impl Instruction {
    fn execute(&self, registers: &mut [i32; 4], pc: usize) -> usize {
        match self {
            Instruction::Cpy(src, target) => {
                let val = match src {
                    ValueType::Num(n) => *n,
                    ValueType::Register(r) => registers[*r as usize],
                };
                registers[*target] = val;
                pc + 1
            }
            Instruction::Inc(target) => {
                registers[*target] += 1;
                pc + 1
            }
            Instruction::Dec(target) => {
                registers[*target] -= 1;
                pc + 1
            }
            Instruction::Jnz(cond, offset) => {
                let cond_val = match cond {
                    ValueType::Num(n) => *n,
                    ValueType::Register(r) => registers[*r as usize],
                };

                if cond_val != 0 {
                    (pc as i32 + offset) as usize
                } else {
                    pc + 1
                }
            }
        }
    }
}

fn to_register_index(c: char) -> usize {
    (c as u8 - 97) as usize
}

fn solve_for_input(path: String) -> i32 {
    let content = fs::read_to_string(path)
        .expect("Should have been able to read the file {path}");

let mut registers = [0; 4];
let instructions: Vec<Instruction> = content.lines()
    .map(str::trim)
    .map(str::split_whitespace)
    .map(|x| x.collect::<Vec<_>>())
    .map(|x| {
        match x[..] {
            ["cpy", x, register] => {
                Instruction::Cpy(
                    ValueType::from_string(x),
                    to_register_index(register.chars().nth(0).unwrap())
                )
            },
            ["inc", register] => {
                Instruction::Inc(to_register_index(register.chars().nth(0).unwrap()))
            },
            ["dec", register] => {
                Instruction::Dec(to_register_index(register.chars().nth(0).unwrap()))
            },
            ["jnz", x, y] => {
                Instruction::Jnz(ValueType::from_string(x), y.parse::<i32>().unwrap())
            },
            _ => panic!()
        }
    })
    .collect::<Vec<_>>();

    let mut index: usize = 0;

    loop {
        match instructions.get(index) {
            None => break,
            Some(instruction) => index = instruction.execute(&mut registers, index),
        }
    }

    registers[0]
}



pub fn main() {
    println!("Validation input : {}", solve_for_input(String::from("./inputs/day12_example.txt")));
    println!("Validation input : {}", solve_for_input(String::from("./inputs/day12_input.txt")));
}
