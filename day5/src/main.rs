type Range = (u64, u64);

fn parse(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|l| {
            let (start, end) = l.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let ingredients = ingredients.lines().map(|l| l.parse().unwrap()).collect();

    (ranges, ingredients)
}

fn part1(ranges: &[Range], ingredients: &[u64]) -> u64 {
    let mut total = 0;
    for i in ingredients {
        for (start, end) in ranges {
            if start <= i && i <= end {
                total += 1;
                break;
            }
        }
    }
    total
}

fn try_merge_ranges(r1: Range, r2: Range) -> Option<Range> {
    let (s1, e1) = r1;
    let (s2, e2) = r2;
    if s1 <= e2 && s2 <= e1 {
        let start = u64::min(s1, s2);
        let end = u64::max(e1, e2);
        Some((start, end))
    } else {
        None
    }
}

fn find_mergeable(ranges: &[Range], to_merge: Range, ignore_idx: usize) -> Option<(usize, Range)> {
    ranges
        .iter()
        .enumerate()
        .filter_map(|(idx, r)| {
            // println!("{idx} {ignore_idx}");
            if idx == ignore_idx {
                None
            } else if let Some(merged) = try_merge_ranges(*r, to_merge) {
                Some((idx, merged))
            } else {
                None
            }
        })
        .next()
}

fn part2(ranges: &[Range]) -> u64 {
    let mut merged_ranges = Vec::new();
    for r in ranges {
        if let Some((idx, mut merged)) = find_mergeable(&mut merged_ranges, *r, usize::MAX) {
            merged_ranges[idx] = merged;
            while let Some((new_idx, new_merged)) = find_mergeable(&mut merged_ranges, merged, idx)
            {
                merged_ranges[idx] = new_merged;
                merged_ranges.swap_remove(new_idx);
                merged = new_merged;
            }
        } else {
            merged_ranges.push(*r)
        }
    }

    merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

fn main() {
    let raw_input = include_str!("../input");
    let (ranges, ingredients) = parse(raw_input);

    let p1 = part1(&ranges, &ingredients);
    println!("Part 1: {p1}");

    let p2 = part2(&ranges);
    println!("Part 2: {p2}");
}
