use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

fn parse(line: &str) -> Result<&str, Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(?P<cap>.*)"#).unwrap();
    }
    let captures = RE.captures(line).unwrap();
    let result = captures.name("cap").unwrap().as_str();
    Ok(result)
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Dir {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

impl Dir {
    pub fn rotate(&self, clockwise: i32) -> Dir {
        match (*self as i32 + clockwise) % 4 {
            0 => Dir::Top,
            1 => Dir::Right,
            2 => Dir::Bottom,
            3 => Dir::Left,
            _ => unreachable!(),
        }
    }

    pub fn rotation(d1: &Dir, d2: &Dir) -> i32 {
        *d2 as i32 - *d1 as i32
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Flip {
    Yes,
    No,
}

impl Flip {
    fn flip(&self) -> Flip {
        match self {
            Flip::Yes => Flip::No,
            Flip::No => Flip::Yes,
        }
    }
}

type Borders = HashMap<String, Vec<(Dir, Flip, u64)>>;

fn extract(buffer: &str) -> Result<(Borders, HashMap<u64, Vec<String>>), Box<dyn Error>> {
    let mut borders: HashMap<String, Vec<(Dir, Flip, u64)>> = HashMap::new();
    let mut tiles = HashMap::new();

    for tile_buffer in buffer.trim().split("\n\n") {
        let lines = tile_buffer.lines().collect::<Vec<_>>();
        let tile_id = &lines[0][5..lines[0].len() - 1].parse::<u64>()?;
        let tile_contents = lines[1..]
            .iter()
            .map(|x| String::from(*x))
            .collect::<Vec<_>>();

        tiles.insert(*tile_id, tile_contents.clone());

        let top = borders
            .entry(String::from(&tile_contents[0]))
            .or_insert(vec![]);
        top.push((Dir::Top, Flip::No, *tile_id));

        let top_flipped = borders
            .entry(tile_contents[0].chars().rev().collect::<String>())
            .or_insert(vec![]);
        top_flipped.push((Dir::Top, Flip::Yes, *tile_id));

        let bottom = borders
            .entry(String::from(&tile_contents[tile_contents.len() - 1]))
            .or_insert(vec![]);
        bottom.push((Dir::Bottom, Flip::No, *tile_id));

        let bottom_flipped = borders
            .entry(
                tile_contents[tile_contents.len() - 1]
                    .chars()
                    .rev()
                    .collect::<String>(),
            )
            .or_insert(vec![]);
        bottom_flipped.push((Dir::Bottom, Flip::Yes, *tile_id));

        let mut left_root = String::from("");
        tile_contents
            .iter()
            .map(|x| &x[0..1])
            .fold(&mut left_root, |a, x| {
                a.push_str(x);
                a
            });
        let left = borders.entry(left_root.clone()).or_insert(vec![]);
        left.push((Dir::Left, Flip::No, *tile_id));

        let left_flipped = borders
            .entry(left_root.chars().rev().collect::<String>())
            .or_insert(vec![]);
        left_flipped.push((Dir::Left, Flip::Yes, *tile_id));

        let mut right_root = String::from("");
        tile_contents
            .iter()
            .map(|x| &x[x.len() - 1..])
            .fold(&mut right_root, |a, x| {
                a.push_str(x);
                a
            });
        let right = borders.entry(right_root.clone()).or_insert(vec![]);
        right.push((Dir::Right, Flip::No, *tile_id));

        let right_flipped = borders
            .entry(right_root.chars().rev().collect::<String>())
            .or_insert(vec![]);
        right_flipped.push((Dir::Right, Flip::Yes, *tile_id));
    }

    Ok((borders, tiles))
}

fn solve1(buffer: &str) -> Result<String, Box<dyn Error>> {
    let (borders, _tiles) = extract(buffer)?;

    let mut tile_counts = HashMap::new();
    for (border, tiles) in &borders {
        for tile in tiles {
            let entry = tile_counts.entry(tile.2).or_insert(0);
            if tiles.len() == 1 {
                *entry += 1;
            }
        }
    }

    Ok(tile_counts
        .iter()
        .filter_map(|(x, y)| if *y == 4 { Some(x) } else { None })
        .product::<u64>()
        .to_string())
}


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Transform {
    clockwise: i32,
    flip: bool
}

impl Transform {

    fn id() -> Transform {
        Transform { clockwise: 0, flip: false }
    }

    fn from(f1: Flip, f2: Flip, d1: Dir, d2: Dir) -> Transform {
        dbg!(d1, d2, Dir::rotation(&d2, &d1));
        Transform { clockwise: Dir::rotation(&d2, &d1), flip: f1 != f2 }
    }

    fn apply(&self, f: Flip, d: Dir) -> (Flip, Dir) {
        // Rotate first, then flip and *also adjust the direction*
        // I only flip horizontally

        let d = d.rotate(self.clockwise);

        match (self.flip, d) {
            (true, Dir::Top) | (true, Dir::Bottom) => (Flip::Yes, d),
            (true, Dir::Right) | (true, Dir::Left) => (Flip::No, d.rotate(2)),
            (false, d) => (Flip::No, d),
        }
    }

    fn original(&self, d: Dir) -> (Flip, Dir) {
        unimplemented!()
    }
}

fn solve2(buffer: &str) -> Result<String, Box<dyn Error>> {
    let (borders, tiles) = extract(buffer)?;
    let total_tiles = tiles.len();
    let side = (total_tiles as f64).sqrt() as usize;

    let border_counts = borders.iter().map(|(x, y)| (x, y.len())).collect::<HashMap<_, _>>();
    let mut tile_counts = HashMap::new();

    let mut tile_borders: HashMap<u64, HashMap<(Flip, Dir), &str>> = HashMap::new();

    for (border, tiles) in &borders {
        for tile in tiles {
            let entry = tile_counts.entry(tile.2).or_insert(0);
            if tiles.len() == 1 {
                *entry += 1;
            }

            let hash_entry = tile_borders.entry(tile.2).or_insert(HashMap::new());
            hash_entry.insert((tile.1, tile.0), border);
        }
    }

    let mut corners = tile_counts
        .iter()
        .filter_map(|(x, y)| if *y == 4 { Some(x) } else { None })
        .collect::<Vec<_>>();
    corners.sort();
    let mut grid = vec![vec![(0, Transform::id()); side]; side];
    grid[0][0].0 = *corners[1];

    let mut current_sides = HashSet::new();
    for (border, tiles) in &borders {
        for tile in tiles {
            if tile.2 == grid[0][0].0 && tiles.len() == 2 {
                current_sides.insert(tile.0);
            }
        }
    }

    let mut rotation = 0;
    let desired_sides = vec![Dir::Left, Dir::Bottom].into_iter().collect::<HashSet<Dir>>();

    while desired_sides != current_sides {
        rotation += 1;
        current_sides = current_sides.iter().map(|x| x.rotate(1)).collect::<HashSet<Dir>>();
    }

    // Figure out orientation of the top left tile
    // grid[0][0].1 = Transform { flip: false, clockwise: rotation };
    grid[0][0].1 = Transform::from(Flip::No, Flip::Yes, Dir::Bottom, Dir::Top);
    let mut prev_tile = grid[0][0].clone();

    for (i, col) in grid[0].iter_mut().enumerate() {
        if i != 0 {
            let i = i + 1;
            let side_details = prev_tile.1.invert(Dir::Right);
            dbg!(side_details);

            let side = tile_borders[&prev_tile.0][&side_details];

            let cur_tile = borders[side].iter().filter(|x| x.2 != prev_tile.0).next().unwrap();
            dbg!(cur_tile);

            col.1 = Transform::from(cur_tile.1, Flip::No, cur_tile.0, Dir::Left);
            col.0 = cur_tile.2;
            dbg!(col.1);
        }

        prev_tile = col.clone();
    }

    for row in grid {
        for col in row {
            print!("{:5?} ", col);
        }
        println!();
    }

    Ok(format!("Solution 2: {}", parse(buffer)?))
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::Instant::now();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "2" {
        println!("{}", solve2(&buffer)?);
    } else {
        println!("{}", solve1(&buffer)?);
    }

    eprintln!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn test1() {
        // dbg!(solve1(INPUT));
    }

    #[test]
    fn test2() {
        dbg!(solve2(INPUT));
    }
}
