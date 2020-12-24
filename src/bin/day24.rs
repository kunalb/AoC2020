use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::io::{self, Read};

type Floor = HashSet<(i32, i32)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    E = 0,
    SE = 1,
    SW = 2,

    W = 3,
    NW = 4,
    NE = 5,
}

impl Dir {
    fn axis(&self) -> usize {
        (*self as usize) % 3
    }

    fn step(&self) -> i32 {
        if (*self as usize) < 3 {
            1
        } else {
            -1
        }
    }
}

fn tokenize(line: &str) -> Vec<Dir> {
    let chars = line.chars().collect::<Vec<char>>();
    let mut result = vec![];

    let mut i = 0;
    while i < chars.len() {
        result.push(match chars[i] {
            'e' => Dir::E,
            'w' => Dir::W,
            's' => {
                i += 1;
                match chars[i] {
                    'e' => Dir::SE,
                    'w' => Dir::SW,
                    _ => unreachable!(),
                }
            }
            'n' => {
                i += 1;
                match chars[i] {
                    'e' => Dir::NE,
                    'w' => Dir::NW,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        });

        i += 1;
    }

    result
}

// excluding multiplying by cos30 for y, multiplying by 2 for x to avoid fractions
fn normalize(pt: [i32; 3]) -> (i32, i32) {
    (pt[0] * 2 + pt[1] - pt[2], pt[1] + pt[2])
}

fn identify(steps: &Vec<Dir>) -> [i32; 3] {
    let mut pos = [0; 3];
    for step in steps {
        pos[step.axis()] += step.step();
    }
    pos
}

fn flip(buffer: &str) -> HashSet<(i32, i32)> {
    let mut floor = HashSet::new();
    for line in buffer.lines() {
        let steps = tokenize(line);
        let loc = identify(&steps);
        let pt = normalize(loc);

        if floor.contains(&pt) {
            floor.remove(&pt);
        } else {
            floor.insert(pt);
        }
    }

    floor
}

fn solve1(buffer: &str) -> Result<usize, Box<dyn Error>> {
    let floor = flip(buffer);
    Ok(floor.len())
}

const OFFSETS: [(i32, i32); 6] = [(-1, -1), (-2, 0), (-1, 1), (1, 1), (2, 0), (1, -1)];

fn black_neighbors((x, y): (i32, i32), floor: &Floor) -> usize {
    OFFSETS
        .iter()
        .filter(|(dx, dy)| floor.contains(&(x + dx, y + dy)))
        .count()
}

fn day(floor: &Floor) -> Floor {
    let mut result = HashSet::new();

    for black_tile in floor {
        let bn = black_neighbors(*black_tile, floor);
        if bn > 0 && bn < 3 {
            result.insert(*black_tile);
        }

        for (dx, dy) in OFFSETS.iter() {
            let white_tile = (black_tile.0 + dx, black_tile.1 + dy);
            let bn = black_neighbors(white_tile, floor);
            if bn == 2 {
                result.insert(white_tile);
            }
        }
    }

    result
}

fn solve2(buffer: &str) -> Result<usize, Box<dyn Error>> {
    let mut floor = flip(buffer);

    for _ in 0..100 {
        floor = day(&floor);
    }

    Ok(floor.len())
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

    const INPUT: &'static str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test1b() {
        assert_eq!(solve1(INPUT).unwrap(), 10);
    }
}
