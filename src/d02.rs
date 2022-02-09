use regex::Regex;

mod utils;

const DAY_ID: utils::DayIdType = 2;

#[derive(Debug)]
struct PasswordRule {
    min: usize,
    max: usize,
    ch: char,
    password: String,
}

impl From<&str> for PasswordRule {
    fn from(line: &str) -> Self {
        let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
        if let Some(cap) = re.captures_iter(line).next() {
            return Self {
                min: cap[1].parse::<usize>().unwrap(),
                max: cap[2].parse::<usize>().unwrap(),
                ch: cap[3].chars().next().unwrap(),
                password: cap[4].to_string(),
            };
        }
        unreachable!();
    }
}

impl PasswordRule {
    pub fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.ch).count();
        count >= self.min && count <= self.max
    }

    pub fn is_valid_special(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        let a = chars[self.min - 1];
        let b = chars[self.max - 1];
        (a == self.ch && b != self.ch) || (b == self.ch && a != self.ch)
    }
}

fn parse_input(data: &str) -> Vec<PasswordRule> {
    data.lines().map(|x| x.into()).collect()
}

fn solve_part1(data: &[PasswordRule]) -> usize {
    data.iter().filter(|x| x.is_valid()).count()
}

fn solve_part2(data: &[PasswordRule]) -> usize {
    data.iter().filter(|x| x.is_valid_special()).count()
}

generate_main!();

generate_tests!(2, 1);
