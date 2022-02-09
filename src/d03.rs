mod utils;

const DAY_ID: utils::DayIdType = 3;

fn parse_input(data: &str) -> Vec<&str> {
    data.lines().collect()
}

fn count_hits(data: &[&str], right: usize, down: usize) -> usize {
    let width = data[0].len();

    let (hits, _) =
        data.iter()
            .skip(down)
            .step_by(down)
            .fold((0usize, 0usize), |(hits, column), row| {
                let new_column = (column + right) % width;
                let new_hit = if row.chars().nth(new_column).unwrap() == '#' {
                    1
                } else {
                    0
                };
                (hits + new_hit, new_column)
            });

    hits
}

fn solve_part1(data: &[&str]) -> usize {
    count_hits(data, 3, 1)
}

fn solve_part2(data: &[&str]) -> usize {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|&(right, down)| count_hits(data, right, down))
        .product()
}

generate_main!();

generate_tests!(7, 336);
