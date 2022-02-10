mod utils;

const DAY_ID: utils::DayIdType = 9;

#[cfg(not(test))]
const PREAMBLE_LEN: usize = 25;

#[cfg(test)]
const PREAMBLE_LEN: usize = 5;

fn parse_input(data: &str) -> Vec<u64> {
    data.lines().map(|x| x.parse().unwrap()).collect()
}

fn is_pre_sum(data: &[u64], index: usize) -> bool {
    let num_in_question = data[index];

    let start = index - PREAMBLE_LEN;

    for x in start..index {
        for y in start..index {
            if x == y {
                continue;
            }
            if data[x] + data[y] == num_in_question {
                return true;
            }
        }
    }

    false
}

fn part1(data: &[u64]) -> (usize, u64) {
    for i in PREAMBLE_LEN..data.len() {
        let num = data[i];
        if !is_pre_sum(data, i) {
            return (i, num);
        }
    }
    unreachable!();
}

fn solve_part1(data: &[u64]) -> u64 {
    part1(data).1
}

fn solve_part2(data: &[u64]) -> u64 {
    let (max_i, result) = part1(data);

    // find a contiguous set of at least two numbers in your list which sum to the invalid number from step 1.
    let mut r: (usize, usize) = (0, 0);
    'outer: for i in 0..max_i {
        let mut sum: u64 = 0;
        let mut pos = i;
        while sum <= result && pos <= max_i {
            sum += data[pos];
            if sum == result && pos != i {
                r = (i, pos);
                break 'outer;
            }
            pos += 1;
        }
    }
    let v: Vec<u64> = data.iter().take(r.1 + 1).skip(r.0).copied().collect();
    let min = v.iter().min().unwrap();
    let max = v.iter().max().unwrap();
    min + max
}

generate_main!();

generate_tests!(127, 62);
