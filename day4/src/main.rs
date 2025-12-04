use ndarray::prelude::*;

fn parse_input(input: &str) -> Array2<bool> {
    let vec: Vec<bool> = input
        .lines()
        .flat_map(|l| l.chars().map(|c| c == '@'))
        .collect();
    let rows = input.lines().count();
    let cols = input.lines().next().map(|l| l.chars().count()).unwrap();
    Array2::from_shape_vec((rows, cols), vec).unwrap()
}

fn get_accessible(grid: &Array2<bool>) -> (u64, Array2<bool>) {
    let mut accessible: Array2<bool> = Array2::from_elem(grid.raw_dim(), false);
    let mut total = 0;
    let (rows, cols) = grid.dim();
    for ((r, c), b) in grid.indexed_iter() {
        if !b {
            continue;
        }

        let r_min = r.checked_sub(1).unwrap_or(r);
        let r_max = (r + 2).min(rows);
        let c_min = c.checked_sub(1).unwrap_or(c);
        let c_max = (c + 2).min(cols);
        let zone = grid.slice(s![r_min..r_max, c_min..c_max]);
        let s = zone.iter().filter(|b| **b).count();

        if s < 5 {
            accessible[[r, c]] = true;
            total += 1;
        }
    }

    (total, accessible)
}

fn part1(grid: &Array2<bool>) -> u64 {
    let (total, _) = get_accessible(grid);
    total
}

fn part2(mut grid: Array2<bool>) -> u64 {
    let mut changed = 1;
    let mut total_removed = Array2::from_elem(grid.raw_dim(), false);
    while changed != 0 {
        let change_mask: Array2<bool>;
        (changed, change_mask) = get_accessible(&grid);
        grid ^= &change_mask;
        total_removed |= &change_mask;
    }

    total_removed.mapv(|b| b as u64).sum()
}

fn main() {
    let raw_input = include_str!("../input");
    let input = parse_input(raw_input);

    let p1 = part1(&input);
    println!("Part 1: {p1}");

    let p2 = part2(input);
    println!("Part 2: {p2}");
}
