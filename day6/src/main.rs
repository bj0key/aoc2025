fn split_input(input: &str) -> (&str, &str) {
    let op_line = input.lines().last().unwrap();
    // offset_from is unsafe cause something something provenenace,
    // everything from input so we're A-OK
    let num_lines_len = unsafe { op_line.as_ptr().offset_from(input.as_ptr()) };
    let num_lines = &input[..num_lines_len as usize];
    (num_lines, op_line)
}

fn parse_ops(line: &str) -> Vec<u8> {
    line.split_ascii_whitespace()
        .map(|op| *op.as_bytes().first().unwrap())
        .collect()
}

fn parse_numbers_p1(lines: &str) -> Vec<Vec<u16>> {
    let n_rows = lines
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .count();
    let mut line_iters = lines
        .lines()
        .map(|l| l.split_ascii_whitespace())
        .collect::<Vec<_>>();

    let mut cols = vec![vec![]; n_rows];
    for r in 0..n_rows {
        for l in line_iters.iter_mut() {
            cols[r].push(l.next().unwrap().parse().unwrap());
        }
    }
    cols
}

fn parse_numbers_p2(lines: &str) -> Vec<Vec<u16>> {
    // We could do something clever, *or* we could just
    // transpose the whole input and then parse line-by-line

    // Just straight-up transposing all of the input lines
    let mut transposed = String::with_capacity(lines.len());
    let line_len = lines.lines().next().unwrap().len();

    let mut lines: Vec<_> = lines.lines().map(|l| l.chars()).collect();
    for _ in 0..line_len {
        for l in lines.iter_mut() {
            transposed.push(l.next().unwrap());
        }
        transposed.push('\n');
    }

    // Parsing the transposed data
    // Now each line contains 1 number, except empty lines
    // which separate the groups
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for line in transposed.lines().map(|l| l.trim()) {
        if line.is_empty() && !group.is_empty() {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(line.parse().unwrap());
        }
    }
    if !group.is_empty() {
        groups.push(group);
    }
    groups
}

fn total(nums: &[Vec<u16>], ops: &[u8]) -> u64 {
    let mut total = 0;
    for (num, op) in nums.iter().zip(ops) {
        total += match op {
            b'+' => num.iter().fold(0u64, |a, b| a + *b as u64),
            b'*' => num.iter().fold(1u64, |a, b| a * *b as u64),
            _ => unreachable!(),
        }
    }
    total
}

fn main() {
    let raw_input = include_str!("../input");

    let (num_lines, op_line) = split_input(raw_input);

    let ops = parse_ops(op_line);

    let nums_p1 = parse_numbers_p1(num_lines);
    let p1 = total(&nums_p1, &ops);
    println!("Part 1: {p1}");

    let nums_p2 = parse_numbers_p2(num_lines);
    let p2 = total(&nums_p2, &ops);
    println!("Part 2: {p2}");
}
