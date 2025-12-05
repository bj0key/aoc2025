type Range = std::ops::RangeInclusive<u64>;

fn parse(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|l| {
            let (start, end) = l.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    let ingredients = ingredients.lines().map(|l| l.parse().unwrap()).collect();

    (ranges, ingredients)
}

fn part1(ranges: &[Range], ingredients: &[u64]) -> u64 {
    let mut total = 0;
    for i in ingredients {
        for r in ranges {
            if r.contains(i) {
                total += 1;
                break;
            }
        }
    }
    total
}

fn try_merge_ranges(r1: &Range, r2: &Range) -> Option<Range> {
    if *r1.start() <= *r2.end() && *r2.start() <= *r1.end() + 1 {
        let start = *r1.start().min(r2.start());
        let end = *r1.end().max(r2.end());
        Some(start..=end)
    } else {
        None
    }
}

fn find_mergeable<'a>(
    ranges: &'a mut [Range],
    to_merge: &Range,
    ignore_idx: usize,
) -> Option<(&'a mut Range, usize, Range)> {
    ranges
        .iter_mut()
        .enumerate()
        .filter_map(|(idx, r)| {
            // println!("{idx} {ignore_idx}");
            if idx == ignore_idx {
                None
            } else if let Some(merged) = try_merge_ranges(r, to_merge) {
                Some((r, idx, merged))
            } else {
                None
            }
        })
        .next()
}

fn part2(ranges: &[Range]) -> u64 {
    let mut merged = Vec::new();
    for r in ranges {
        if let Some((m, idx, new)) = find_mergeable(&mut merged, r, usize::MAX) {
            *m = new;
            let mut to_merge = m.clone();
            let mut to_merge_idx = idx;

            while let Some((m, idx2, new)) = find_mergeable(&mut merged, &to_merge, to_merge_idx) {
                *m = new.clone();
                merged.remove(to_merge_idx);
                to_merge_idx = if idx2 > to_merge_idx {
                    idx2 - 1
                } else {
                    idx2
                };
                to_merge = new;
            }
        } else {
            merged.push(r.clone())
        }
    }

    merged.iter().map(|r| r.end() - r.start() + 1).sum()
}

fn main() {
    let raw_input = include_str!("../input");
    let (ranges, ingredients) = parse(raw_input);

    let p1 = part1(&ranges, &ingredients);
    println!("Part 1: {p1}");

    let p2 = part2(&ranges);
    println!("Part 2: {p2}");
}
