mod utils;

const DAY_ID: utils::DayIdType = 1;

fn parse_input(data: &str) -> Vec<u32> {
    data.lines().map(|x| x.parse::<u32>().unwrap()).collect()
}

fn solve_part1(data: &[u32]) -> u32 {
    for a in data.iter() {
        for b in data.iter() {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    unreachable!();
}

fn solve_part2(data: &[u32]) -> u32 {
    for a in data.iter() {
        for b in data.iter() {
            for c in data.iter() {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    unreachable!();
}

generate_main!();

generate_tests!(514579, 241861950);
