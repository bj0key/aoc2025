use itertools::Itertools;
use std::fmt::Debug;

#[derive(Debug)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}

fn parse(input: &str) -> Vec<Machine> {
    let mut buf = Vec::new();
    input
        .lines()
        .map(|line| {
            buf.clear();
            buf.extend(line.split_ascii_whitespace());
            let [target_str, buttons_strs @ .., joltages_str] = buf.as_slice() else {
                panic!("Couldn't split line enough: {buf:?}");
            };

            let target = target_str[1..target_str.len()]
                .chars()
                .map(|c| c == '#')
                .collect();

            let buttons = buttons_strs
                .iter()
                .map(|s| {
                    s[1..s.len() - 1]
                        .split(',')
                        .map(|n| n.parse().unwrap())
                        .collect()
                })
                .collect();

            let joltages = joltages_str[1..joltages_str.len() - 1]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();

            Machine {
                target,
                buttons,
                joltages,
            }
        })
        .collect()
}

fn part1(machines: &[Machine]) -> u64 {
    let mut total = 0;
    'm: for m in machines {
        let mut actual = vec![false; m.target.len()];
        for combo_len in 0..m.target.len() {
            for combo in m.buttons.iter().combinations(combo_len) {
                actual.fill(false);
                for &i in combo.iter().flat_map(|v| v.iter()) {
                    actual[i] = !actual[i];
                }
                if m.target == actual {
                    total += combo_len as u64;
                    continue 'm;
                }
            }
        }
        unreachable!("Button combo loop should always exit via the continue");
    }
    total
}

fn part2(machines: &[Machine]) -> u64 {
    todo!()
}

fn main() {
    let raw_input = include_str!("../example");
    let input = parse(&raw_input);

    let p1 = part1(&input);
    println!("Part 1: {p1}");

    let p2 = part2(&input);
    println!("Part 2: {p2}");
}
