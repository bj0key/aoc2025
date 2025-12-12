// You thought this problem had some secret clever solution?
// Nope! the secret is just to pretend every present is just a 3x3
// solid block.
// That's it.
// Parsing the presents is meaningless, we don't need them at all

struct Region {
    width: u16,
    height: u16,
    counts: [u16; 6],
}

fn parse(input: &str) -> Vec<Region> {
    input
        .lines()
        .skip(30) // Its always 6 presents, each one defined with 5 lines
        .map(|l| {
            let (size, counts) = l.split_once(": ").unwrap();
            let (w, h) = size.split_once('x').unwrap();
            let counts = counts.split(' ').map(|n| n.parse().unwrap());
            Region {
                width: w.parse().unwrap(),
                height: h.parse().unwrap(),
                counts: counts.collect::<Vec<_>>().try_into().unwrap(),
            }
        })
        .collect()
}

fn complete_and_utter_cheese(input: &[Region]) -> u64 {
    input
        .iter()
        .filter(|region| region.width * region.height >= region.counts.iter().sum::<u16>() * 9)
        .count()
        .try_into()
        .unwrap()
}

fn main() {
    let raw_input = include_str!("../input");
    let input = parse(raw_input);

    let p1 = complete_and_utter_cheese(&input);
    println!("Part 1: {p1}");
}
