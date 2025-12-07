fn parse(input: &str) -> (usize, Vec<Vec<bool>>) {
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let start = first.find('S').unwrap();
    let splits = lines
        .map(|l| l.chars().map(|c| c == '^').collect())
        .collect();
    (start, splits)
}

fn part1(start: usize, splitters: &[Vec<bool>]) -> u64 {
    let cols = splitters[0].len();
    let mut beams = vec![false; cols];
    let mut total_splits = 0;
    beams[start] = true;
    for splitter_row in splitters {
        for c in 0..cols {
            if beams[c] && splitter_row[c] {
                beams[c] = false;
                if c > 0 {
                    beams[c - 1] = true;
                }
                if c < cols - 1 {
                    beams[c + 1] = true;
                }
                total_splits += 1;
            }
        }
    }
    total_splits
}

fn part2(start: usize, splitters: &[Vec<bool>]) -> u64 {
    let cols = splitters[0].len();
    let mut timelines = vec![0; cols];
    let mut next_timelines = vec![0; cols];
    timelines[start] = 1;
    next_timelines[start] = 1;
    for splitter_row in splitters {
        for c in 0..cols {
            if splitter_row[c] {
                next_timelines[c] = 0;
                if c > 0 {
                    next_timelines[c - 1] += timelines[c];
                }
                if c < cols - 1 {
                    next_timelines[c + 1] += timelines[c];
                }
            }
        }
        timelines.copy_from_slice(&next_timelines);
    }
    timelines.iter().sum()
}

fn main() {
    let raw_input = include_str!("../input");
    let (start, splitters) = parse(raw_input);
    let p1 = part1(start, &splitters);
    println!("Part 1: {p1}");

    let p2 = part2(start, &splitters);
    println!("Part 2: {p2}");
}
