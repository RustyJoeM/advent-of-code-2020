use std::collections::HashMap;

use regex::Regex;

mod utils;

const DAY_ID: utils::DayIdType = 7;

#[derive(Debug, Default, Clone)]
struct Container {
    pub color: String,
    pub contents: Vec<(usize, String)>,
}

struct RuleMap {
    pub rules: HashMap<String, Container>,
}

impl RuleMap {
    pub fn new() -> RuleMap {
        Self {
            rules: HashMap::default(),
        }
    }

    pub fn can_traverse_to_golden_shiny(&self, container: &Container) -> bool {
        if container.color == "shiny gold" {
            return true;
        }
        for item in &container.contents {
            if item.1 == "shiny gold" {
                return true;
            }
            if self.rules.contains_key(&item.1)
                && self.can_traverse_to_golden_shiny(&self.rules[&item.1])
            {
                return true;
            }
        }
        false
    }

    pub fn count_bags(&self, container: &Container) -> usize {
        let mut sum = 1;
        for item in &container.contents {
            if self.rules.contains_key(&item.1) {
                let count = item.0;
                let sub_rule = &self.rules[&item.1];
                sum += count * self.count_bags(sub_rule);
            }
        }
        sum
    }
}

fn parse_input(data: &str) -> RuleMap {
    let mut rule_map = RuleMap::new();

    let re = Regex::new(r"^([a-z ]+) bags contain (.*).$").unwrap();
    let re_content = Regex::new(r"^([0-9]+) (.*) bag[s]?$").unwrap();

    for line in data.lines() {
        let mut container: Container = Default::default();
        for capture in re.captures_iter(line) {
            container.color = capture[1].to_string();

            let bags_inside: Vec<&str> = capture[2].split(", ").collect();
            for item in bags_inside {
                for inner in re_content.captures_iter(item) {
                    let cnt: usize = inner[1].parse().unwrap();
                    let bag = inner[2].to_string();

                    container.contents.push((cnt, bag));
                }
            }
        }
        rule_map.rules.insert(container.color.clone(), container);
    }

    rule_map
}

fn solve_part1(data: &RuleMap) -> usize {
    data.rules
        .values()
        .filter(|x| data.can_traverse_to_golden_shiny(x))
        .count()
        - 1 // shiny gold itself
}

fn solve_part2(data: &RuleMap) -> usize {
    let golden_rule = &data.rules["shiny gold"];
    data.count_bags(golden_rule) - 1
}

generate_main!();

generate_tests!(4, 32);
