fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(u8::try_from).unwrap().unwrap())
                .collect()
        })
        .collect()
}

// Hand-wrote my own function to find the max + max idx,
// since .iter().max() returns the last maximum by default,
// plus we know 9 is the biggest number and so can early return once
// we find one.

fn next_biggest(bank: &[u8], headroom: usize) -> (u8, usize) {
    let mut max_idx = 0;
    let mut max = 0;
    for (idx, val) in bank[..=bank.len() - headroom].iter().copied().enumerate() {
        if val > max {
            max = val;
            max_idx = idx;
            if val == 9 {
                break;
            }
        }
    }
    (max, max_idx)
}

fn biggest_in_bank(mut bank: &[u8], mut len: usize) -> u64 {
    let mut total = 0;
    while len > 0 {
        total *= 10;
        let (max, max_idx) = next_biggest(bank, len);
        bank = &bank[max_idx + 1..];
        total += max as u64;
        len -= 1;
    }
    total
}

fn part1(input: &[Vec<u8>]) -> u64 {
    input.iter().map(|bank| biggest_in_bank(bank, 2)).sum()
}

fn part2(input: &[Vec<u8>]) -> u64 {
    input.iter().map(|bank| biggest_in_bank(bank, 12)).sum()
}

fn main() {
    let raw_input = include_str!("../input");
    let input = parse_input(raw_input);

    let p1 = part1(&input);
    println!("Part 1: {p1}");

    let p2 = part2(&input);
    println!("Part 2: {p2}");
}
