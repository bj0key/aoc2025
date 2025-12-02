use std::ops::Range;

fn count_digits(n: u64) -> u32 {
    if n == 0 { 1 } else { 1 + n.ilog10() }
}

fn is_repeater_p1(num: u64) -> bool {
    let n_digits = count_digits(num);
    if n_digits % 2 != 0 {
        return false;
    }

    let power_n2 = 10u64.pow(n_digits / 2);
    let first_half = num / power_n2;
    let repeated = first_half + (first_half * power_n2);

    repeated == num
}

fn repeated(n: u64, count: u32) -> u64 {
    let mut total = 0;
    let tens = 10u64.pow(count_digits(n));
    for _ in 0..count {
        total *= tens;
        total += n;
    }
    total
}

fn is_repeater_p2(num: u64) -> bool {
    let n_digits = count_digits(num);
    for seq_len in 1..n_digits {
        if n_digits % seq_len != 0 {
            continue;
        }
        let seq = num % 10u64.pow(seq_len);
        if repeated(seq, n_digits / seq_len) == num {
            return true;
        }
    }
    false
}

fn parse_input(input: &str) -> Vec<Range<u64>> {
    input
        .trim()
        .split(',')
        .map(|r| {
            let (x, y) = r.split_once('-').expect("Couldn't find '-' in range");
            let x: u64 = x.parse().unwrap();
            let y: u64 = y.parse().unwrap();
            x..(y + 1)
        })
        .collect()
}

fn part1(input: &[Range<u64>]) -> u64 {
    let mut total = 0;
    for range in input.iter().cloned() {
        for i in range {
            if is_repeater_p1(i) {
                total += i;
            }
        }
    }
    total
}

fn part2(input: &[Range<u64>]) -> u64 {
    let mut total = 0;
    for range in input.iter().cloned() {
        for i in range {
            if is_repeater_p2(i) {
                total += i;
            }
        }
    }
    total
}

fn main() {
    let raw_input = include_str!("../input");
    let input = parse_input(raw_input);

    let p1 = part1(&input);
    println!("Part 1: {p1}");

    let p2 = part2(&input);
    println!("Part 2: {p2}");
}
