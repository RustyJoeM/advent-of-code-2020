use std::collections::HashSet;

mod utils;

const DAY_ID: utils::DayIdType = 8;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let tokens: Vec<&str> = s.split(' ').collect();
        let val: i32 = tokens[1].parse().unwrap();
        match tokens[0] {
            "acc" => Instruction::Acc(val),
            "jmp" => Instruction::Jmp(val),
            "nop" => Instruction::Nop(val),
            _ => panic!("Unexpected instruction: {}!", tokens[0]),
        }
    }
}

fn parse_input(data: &str) -> Vec<Instruction> {
    data.lines().map(|x| x.into()).collect()
}

#[derive(Debug, Default)]
struct Stack {
    acc: i32,
    pos: i32,
    visited: HashSet<i32>,
}

impl Stack {
    pub fn run_code(&mut self, instructions: &[Instruction]) {
        let mut reentered = false;
        let last_pos = instructions.len() as i32;
        while !reentered && self.pos < last_pos {
            let instr = instructions[self.pos as usize];
            self.visited.insert(self.pos);
            match instr {
                Instruction::Acc(i) => {
                    self.acc += i;
                    self.pos += 1;
                }
                Instruction::Jmp(i) => {
                    self.pos += i;
                }
                Instruction::Nop(_) => {
                    self.pos += 1;
                }
            }
            reentered = self.visited.contains(&self.pos);
        }
    }
}

/// return true when something flipped
fn flip_instruction(v: &mut [Instruction], i: usize) -> bool {
    match v[i] {
        Instruction::Acc(_) => {
            return false;
        }
        Instruction::Jmp(x) => {
            v[i] = Instruction::Nop(x);
        }
        Instruction::Nop(x) => {
            v[i] = Instruction::Jmp(x);
        }
    }
    true
}

fn solve_part1(data: &[Instruction]) -> i32 {
    let mut stack = Stack::default();
    stack.run_code(data);
    stack.acc
}

fn solve_part2(data: &[Instruction]) -> i32 {
    let mut instructions = data.to_vec();

    let last_pos = instructions.len() as i32;

    for i in 0..data.len() {
        let mut stack: Stack = Stack::default();
        if !flip_instruction(&mut instructions, i) {
            continue;
        }
        stack.run_code(&instructions);
        if stack.pos >= last_pos {
            return stack.acc;
        }
        flip_instruction(&mut instructions, i);
    }
    unreachable!();
}

generate_main!();

generate_tests!(5, 8);
