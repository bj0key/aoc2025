use std::collections::BTreeSet;

type Coords = (usize, usize);

fn parse(input: &str) -> Vec<Coords> {
    input
        .lines()
        .map(|l| {
            let (lhs, rhs) = l.split_once(',').unwrap();
            (lhs.parse().unwrap(), rhs.parse().unwrap())
        })
        .collect()
}

fn part1(tiles: &[Coords]) -> u64 {
    let mut max_area = 0;
    for &(x1, y1) in tiles {
        for &(x2, y2) in tiles {
            let width = 1 + x1.abs_diff(x2) as u64;
            let height = 1 + y1.abs_diff(y2) as u64;
            let area = width * height;
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Unset,
    Outside,
    Corner,
    Wall,
}

type Grid = Vec<Vec<Tile>>;

fn draw_line(grid: &mut Grid, start: Coords, end: Coords) {
    let dx = end.0.checked_signed_diff(start.0).unwrap().signum();
    let dy = end.1.checked_signed_diff(start.1).unwrap().signum();
    let mut curr = start;
    while curr != end {
        if grid[curr.1][curr.0] == Tile::Unset {
            grid[curr.1][curr.0] = Tile::Wall;
        };
        curr.0 = curr.0.strict_add_signed(dx);
        curr.1 = curr.1.strict_add_signed(dy);
    }
}

fn get_at_offset(grid: &Grid, pos: Coords, offset: (isize, isize)) -> Option<(Coords, Tile)> {
    let (x, y) = pos;
    let (dx, dy) = offset;
    let (x, y) = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);
    Some(((x, y), *grid.get(y)?.get(x)?))
}

fn flood_fill(grid: &mut Grid, start: Coords, tile: Tile) {
    let mut to_visit = BTreeSet::new();
    to_visit.insert(start);

    while let Some(curr) = to_visit.pop_first() {
        const OFFSETS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

        for offset in OFFSETS {
            if let Some((pos, val)) = get_at_offset(grid, curr, offset)
                && val == Tile::Unset
            {
                to_visit.insert(pos);
            }
        }

        grid[curr.1][curr.0] = tile;
    }
}

fn coord_compression(tiles: &[Coords]) -> (Vec<usize>, Vec<usize>, Vec<(usize, usize)>) {
    let mut xs = Vec::with_capacity(tiles.len());
    let mut ys = Vec::with_capacity(tiles.len());

    for &(x, y) in tiles {
        xs.push(x);
        ys.push(y);
    }

    xs.sort();
    xs.dedup();

    ys.sort();
    ys.dedup();

    let mut compressed = Vec::with_capacity(tiles.len());

    for (x, y) in tiles {
        let x_idx = xs.binary_search(x).unwrap();
        let y_idx = ys.binary_search(y).unwrap();

        compressed.push((x_idx, y_idx));
    }

    (xs, ys, compressed)
}

fn part2(red_tiles: &[Coords]) -> u64 {
    // Step 1: compress coordinates
    // all the coords remain in the same order relative to each other,
    // but their magnitudes are all *drastically* smaller
    let (xs, ys, compressed) = coord_compression(red_tiles);
    let cols = compressed.iter().map(|c| c.0).max().unwrap() + 1;
    let rows = compressed.iter().map(|c| c.1).max().unwrap() + 1;

    // Step 2: grid-ify the coords
    // We can only do this because of the compression trick,
    // without it, this vector blows up to around 10GB in size
    let mut grid = vec![vec![Tile::Unset; cols]; rows];

    // Adding corners (red tiles) and walls (connecting green tiles) to grid
    for window in compressed.windows(2) {
        let &[start, end] = window else { panic!() };
        grid[start.1][start.0] = Tile::Corner;
        grid[end.1][end.0] = Tile::Corner;
        draw_line(&mut grid, start, end);
    }
    draw_line(
        &mut grid,
        *compressed.last().unwrap(),
        *compressed.first().unwrap(),
    );

    // Marking all the tiles that are outside of the shape
    // flood-filling from the 4 far corners seems to do a good enough job
    flood_fill(&mut grid, (0, 0), Tile::Outside);
    flood_fill(&mut grid, (cols - 1, 0), Tile::Outside);
    flood_fill(&mut grid, (0, rows - 1), Tile::Outside);
    flood_fill(&mut grid, (cols - 1, rows - 1), Tile::Outside);

    // Step 3: finally searching for this supposed biggest rectangle
    // This is like part 1, except we also rule out any rectangles that contain any "Outside" tiles
    let mut max_area = 0;

    for &(x1, y1) in compressed.iter() {
        for &(x2, y2) in compressed.iter() {
            let (xmin, xmax) = (x1.min(x2), x1.max(x2));
            let (ymin, ymax) = (y1.min(y2), y1.max(y2));

            // don't forget we compressed the coords, so need to index into xs and ys
            // to get the actual area
            let area = ((xs[xmax] - xs[xmin] + 1) * (ys[ymax] - ys[ymin] + 1)) as u64;

            // only consider areas which are larger than the currently known one, AND
            // where none of the walls of the rectangle touch an outside tile
            if area > max_area
                && (xmin..xmax + 1).all(|x| grid[ymin][x] != Tile::Outside)
                && (xmin..xmax + 1).all(|x| grid[ymax][x] != Tile::Outside)
                && (ymin..ymax + 1).all(|y| grid[y][xmin] != Tile::Outside)
                && (ymin..ymax + 1).all(|y| grid[y][xmax] != Tile::Outside)
            {
                max_area = area;
            }
        }
    }
    max_area
}

fn main() {
    let raw_input = include_str!("../input");
    let input = parse(raw_input);

    let p1 = part1(&input);
    println!("Part 1: {p1}");

    let p2 = part2(&input);
    println!("Part 2: {p2}");
}
