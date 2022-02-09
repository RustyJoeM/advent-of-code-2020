mod utils;

use regex::Regex;
use std::collections::HashMap;

const DAY_ID: utils::DayIdType = 4;

fn str_in_range(s: &str, min: usize, max: usize) -> bool {
    let num = s.parse::<usize>().unwrap();
    num >= min && num <= max
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum FieldType {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}

struct Field<'a> {
    kind: FieldType,
    value: &'a str,
}

impl<'a> Field<'a> {
    pub fn new(s: &'a str) -> Self {
        let (kind_str, value) = s.split_once(':').unwrap();
        let kind = match kind_str {
            "byr" => FieldType::BirthYear,
            "iyr" => FieldType::IssueYear,
            "eyr" => FieldType::ExpirationYear,
            "hgt" => FieldType::Height,
            "hcl" => FieldType::HairColor,
            "ecl" => FieldType::EyeColor,
            "pid" => FieldType::PassportId,
            "cid" => FieldType::CountryId,
            _ => unreachable!(),
        };
        Field { kind, value }
    }

    pub fn is_valid(&self) -> bool {
        match self.kind {
            FieldType::BirthYear => str_in_range(self.value, 1920, 2002),
            FieldType::IssueYear => str_in_range(self.value, 2010, 2020),
            FieldType::ExpirationYear => str_in_range(self.value, 2020, 2030),
            FieldType::Height => {
                if self.value.ends_with("cm") {
                    str_in_range(&self.value[0..self.value.len() - 2], 150, 193)
                } else if self.value.ends_with("in") {
                    str_in_range(&self.value[0..self.value.len() - 2], 59, 76)
                } else {
                    false
                }
            }
            FieldType::HairColor => {
                let hair_color: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                hair_color.is_match(self.value)
            }
            FieldType::EyeColor => {
                let eye_colors: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                eye_colors.contains(&self.value)
            }
            FieldType::PassportId => {
                let pid: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
                pid.is_match(self.value)
            }
            FieldType::CountryId => true,
        }
    }
}

#[derive(Default)]
struct Passport<'a> {
    pub fields: HashMap<FieldType, Field<'a>>,
}

impl<'a> Passport<'a> {
    pub fn is_valid(&self) -> bool {
        const EXPECTED_FIELD_COUNT: usize = 8;
        let fields_cnt = self.fields.keys().len();
        fields_cnt == EXPECTED_FIELD_COUNT
            || (fields_cnt == (EXPECTED_FIELD_COUNT - 1)
                && !self.fields.contains_key(&FieldType::CountryId))
    }

    pub fn is_really_valid(&self) -> bool {
        self.is_valid() && self.fields.values().all(Field::is_valid)
    }
}

fn parse_input(data: &str) -> Vec<Passport> {
    let mut passports = Vec::new();

    for group in data.split("\n\n") {
        let mut passport = Passport::default();
        for line in group.lines() {
            let props: Vec<&str> = line.split_whitespace().collect();
            for s in props {
                let field = Field::new(s);
                passport.fields.insert(field.kind.clone(), field);
            }
        }
        passports.push(passport);
    }

    passports
}

fn solve_part1(data: &[Passport]) -> usize {
    data.iter().filter(|p| p.is_valid()).count()
}

fn solve_part2(data: &[Passport]) -> usize {
    data.iter().filter(|p| p.is_really_valid()).count()
}

generate_main!();

generate_tests!(2, 2);
