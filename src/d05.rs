mod utils;

const DAY_ID: utils::DayIdType = 5;

#[derive(Debug, Copy, Clone)]
struct Seat {
    row: u32,
    column: u32,
}

type SeatId = u32;

impl Seat {
    pub fn seat_id(&self) -> SeatId {
        self.row * 8 + self.column
    }
}

impl From<&str> for Seat {
    fn from(s: &str) -> Self {
        const ROW_CHARS: usize = 7;
        // row from first 7 chars
        let mut row = 0;
        let mut mul = 64;
        for ch in s.chars().take(ROW_CHARS) {
            if ch == 'B' {
                row += mul;
            }
            mul /= 2;
        }
        // column from the rest...
        let mut column = 0;
        let mut mul = 4;
        for ch in s.chars().skip(ROW_CHARS) {
            if ch == 'R' {
                column += mul;
            }
            mul /= 2;
        }

        Self { row, column }
    }
}

fn parse_input(data: &str) -> Vec<Seat> {
    data.lines().map(|x| x.into()).collect()
}

fn solve_part1(data: &[Seat]) -> u32 {
    data.iter().map(|x| x.seat_id()).max().unwrap()
}

fn solve_part2(data: &[Seat]) -> u32 {
    let mut seat_ids: Vec<u32> = data.iter().map(|x| x.seat_id()).collect();
    seat_ids.sort_unstable();
    for i in 0..seat_ids.len() - 1 {
        if seat_ids[i] + 1 != seat_ids[i + 1] {
            return seat_ids[i] + 1;
        }
    }
    unreachable!();
}

generate_main!();

generate_tests!(357, 0);
