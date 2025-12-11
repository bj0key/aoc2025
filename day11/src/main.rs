use std::collections::HashMap;

type Devices<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse(input: &str) -> Devices<'_> {
    let mut table = HashMap::new();

    for line in input.lines() {
        let (name, outputs) = line.split_once(": ").unwrap();
        let outputs: Vec<_> = outputs.split_ascii_whitespace().collect();

        let t = table.insert(name, outputs);
        assert!(t.is_none())
    }
    table
}

type Cache<'a> = HashMap<&'a str, u64>;

fn path_count<'a>(map: &Devices<'a>, curr: &'a str, end: &'a str, cache: &mut Cache<'a>) -> u64 {
    if let Some(&n) = cache.get(&curr) {
        return n;
    }

    let Some(nexts) = map.get(curr) else { return 0 };
    let total = if nexts.contains(&end) {
        1
    } else {
        nexts.iter().map(|n| path_count(map, n, end, cache)).sum()
    };

    cache.insert(curr, total);
    total
}

fn part1(input: &Devices<'_>) -> u64 {
    path_count(input, "you", "out", &mut Cache::new())
}

fn part2(input: &Devices<'_>) -> u64 {
    // There are essentially two general paths:
    // 1) svr -> fft -> dac -> out
    // 2) svr -> dac -> fft -> out
    // So for each general path, we can just find the # of paths between each "landmark" device,
    // and multiply them together for the total number of combinations

    let routes = [["svr", "fft", "dac", "out"], ["svr", "dac", "fft", "out"]];
    let mut total = 0;

    for route in routes {
        let mut combos = 1;

        for stretch in route.windows(2) {
            let &[start, end] = stretch else { panic!() };
            let count = path_count(input, start, end, &mut Cache::new());

            combos *= count
        }

        total += combos
    }
    total
}

fn main() {
    let raw_input = include_str!("../input");
    let input = parse(raw_input);

    let p1 = part1(&input);
    println!("Part 1: {p1}");

    let p2 = part2(&input);
    println!("Part 2: {p2}");
}
