// rust-analyzer: ignore

const fn next_parsed_u16(input: &mut &[u8]) -> i16 {
    let mut total = 0;
    while let [first, rest @ ..] = input {
        if *first < b'0' || *first > b'9' {
            break;
        }
        total *= 10;
        total += (*first - b'0') as i16;
        *input = rest;
    }
    total
}

const fn next_dir(input: &mut &[u8]) -> Option<i16> {
    let [first, rest @ ..] = input else {
        return None;
    };
    let value = match *first {
        b'L' => -1,
        b'R' => 1,
        _ => unreachable!(),
    };
    *input = rest;
    Some(value)
}

const fn skip_whitespace(input: &mut &[u8]) {
    while let [first, rest @ ..] = input
        && first.is_ascii_whitespace()
    {
        *input = rest;
    }
}

const fn next_turn(input: &mut &[u8]) -> Option<i16> {
    skip_whitespace(input);
    let Some(dir) = next_dir(input) else {
        return None;
    };
    let amount = next_parsed_u16(input);
    Some(dir * amount)
}

pub const fn part1(mut input: &[u8]) -> u16 {
    let mut dial = 50;
    let mut zeroes = 0;
    while let Some(turn) = next_turn(&mut input) {
        dial += turn;
        dial = dial.rem_euclid(100);
        if dial == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

pub const fn part2(mut input: &[u8]) -> u16 {
    let mut dial = 50;
    let mut zeroes = 0;
    while let Some(turn) = next_turn(&mut input) {
        let dir = turn.signum();
        assert!(dir != 0, "No zero-long turns should exist");
        let mut steps = turn.unsigned_abs();
        while steps > 0 {
            dial += dir;
            dial = dial.rem_euclid(100);
            if dial == 0 {
                zeroes += 1;
            }
            steps -= 1;
        }
    }
    zeroes
}

const INPUT: &'static [u8] = include_bytes!("../input");
pub const PART_1_SOLUTION: u16 = part1(INPUT);
pub const PART_2_SOLUTION: u16 = part2(INPUT);