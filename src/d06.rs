use std::collections::{HashMap, HashSet};

mod utils;

const DAY_ID: utils::DayIdType = 6;

fn parse_input(data: &str) -> Vec<Vec<&str>> {
    data.split("\n\n").map(|s| s.lines().collect()).collect()
}

fn solve_part1(data: &[Vec<&str>]) -> usize {
    let mut total = 0;
    for group in data.iter() {
        let mut questions: HashSet<char> = HashSet::new();
        for person in group.iter() {
            for ch in person.chars() {
                questions.insert(ch);
            }
        }
        total += questions.len();
    }
    total
}

fn solve_part2(data: &[Vec<&str>]) -> usize {
    let mut total = 0;
    for group in data.iter() {
        let mut questions: HashMap<char, usize> = HashMap::new();
        for person in group.iter() {
            for ch in person.chars() {
                *questions.entry(ch).or_insert(0) += 1;
            }
        }
        total += questions.iter().filter(|(_, v)| **v == group.len()).count();
    }
    total
}

generate_main!();

generate_tests!(11, 6);
