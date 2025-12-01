fn parse_input(input: &str) -> Vec<i16> {
    input
        .lines()
        .map(|line| {
            let dir = line.as_bytes()[0];
            let amount: i16 = line[1..].parse().unwrap();
            match dir {
                b'R' => amount,
                b'L' => -amount,
                _ => unreachable!("Line should always start with L or R"),
            }
        })
        .collect()
}

fn part1(input: &[i16]) -> u64 {
    let mut dial = 50;
    let mut zeros = 0;
    for &rot in input {
        dial = (dial + rot).rem_euclid(100);
        if dial == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn part2(input: &[i16]) -> u64 {
    let mut dial = 50;
    let mut zeroes = 0;
    for &rot in input {
        let new_dial = dial + rot;

        let zeroes_passed = match new_dial {
            // We land exactly on 0
            0 => 1,

            // We went above 99
            100.. => new_dial.div_euclid(100).try_into().unwrap(),

            // We went below 0
            ..0 => {
                let mut passed = (new_dial - 1).div_euclid(100).abs().try_into().unwrap();
                // fixes an annoying off-by-one if we started out on a zero
                if dial == 0 {
                    passed -= 1;
                }
                passed
            }

            // We stayed within the 0..100 range of values
            _ => 0,
        };

        zeroes += zeroes_passed;
        dial = new_dial.rem_euclid(100);
    }
    zeroes
}

fn main() {
    let raw_input = include_str!("../input");
    let input = parse_input(raw_input);
    let p1 = part1(&input);
    println!("Part 1: {p1}");

    let p2 = part2(&input);
    println!("Part 2: {p2}");
}
