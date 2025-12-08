#[derive(Debug)]
struct Junction {
    x: u64,
    y: u64,
    z: u64,
}

fn parse(input: &str) -> Vec<Junction> {
    let mut junctions = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let mut split = line.split(',');
        // stabilise .collect_array(), cheers
        let [x, y, z] = [(); 3].map(|_| split.next().unwrap().parse().unwrap());
        junctions.push(Junction { x, y, z });
    }
    junctions
}

fn precalc_idx_combos(junctions: &[Junction]) -> Vec<(u16, u16)> {
    // save a bit of space by working in u16s,
    // we can cast them back to usizes later
    let len = junctions.len();
    let mut v = Vec::with_capacity((len * (len - 1)) / 2);

    let len_u16: u16 = len.try_into().unwrap();
    for i in 1..len_u16 {
        for j in 0..i {
            v.push((i, j));
        }
    }
    v.sort_by_key(|&(i, j)| distance(&junctions[i as usize], &junctions[j as usize]));
    v
}

fn distance(j1: &Junction, j2: &Junction) -> u64 {
    let x = j1.x.abs_diff(j2.x);
    let y = j1.y.abs_diff(j2.y);
    let z = j1.z.abs_diff(j2.z);
    x * x + y * y + z * z
}

fn merge_circuits(circuits: &mut [u16], dest: u16, to_merge: u16) {
    for c in circuits.iter_mut() {
        if *c == to_merge {
            *c = dest;
        }
    }
}

fn all_merged(circuits: &[u16]) -> bool {
    let Some(first) = circuits.first().copied() else {
        return true;
    };
    circuits.iter().all(|n| *n == first)
}

fn part1(junctions: &[Junction]) -> u64 {
    let idx_combos = precalc_idx_combos(junctions);
    
    // circuits[i] represents which circuit junctions[i] is currently on
    // at first every junction has its own circuit
    let mut circuits: Vec<u16> = (0..junctions.len().try_into().unwrap()).collect();
    for i in 0..1000 {
        let [cx, cy] = <[_; 2]>::from(idx_combos[i]).map(|n| circuits[n as usize]);
        if cx != cy {
            merge_circuits(&mut circuits, cx, cy);
        }
    }
    let mut circuit_sizes = vec![0; circuits.len()];
    for c in circuits.iter() {
        circuit_sizes[*c as usize] += 1;
    }
    circuit_sizes.sort();

    let top_3: &[_; 3] = circuit_sizes.last_chunk().unwrap();
    top_3.iter().product()
}

fn part2(junctions: &[Junction]) -> u64 {
    // Part 1, except instead of ending after 1000 iters & multiplying 3,
    // we just keep going until sorted & multiply the final paring's X coords

    let idx_combos = precalc_idx_combos(junctions);
    let mut circuits: Vec<u16> = (0..junctions.len().try_into().unwrap()).collect();

    for combo in idx_combos.iter() {
        let [x, y] = <[_; 2]>::from(*combo).map(|n| n as usize);
        let [cx, cy] = [circuits[x], circuits[y]];
        if cx != cy {
            merge_circuits(&mut circuits, cx, cy);
        }
        if all_merged(&circuits) {
            let jx = &junctions[x];
            let jy = &junctions[y];
            return jx.x * jy.x;
        }
    }

    unreachable!("Circuits somehow failed to fully connect");
}

fn main() {
    let raw_input = include_str!("../input");
    let input = parse(raw_input);
    let p1 = part1(&input);
    println!("Part 1: {p1}");
    let p2 = part2(&input);
    println!("Part 2: {p2}");
}
