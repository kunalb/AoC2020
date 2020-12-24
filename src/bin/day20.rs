use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::default::Default;
use std::env;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::io::{self, Read};
use std::str::FromStr;

const S: usize = 10;

const MONSTER: &'static str = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord, Default)]
struct TileId(u64);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Dir {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Flipped {
    Yes = 1,
    No = 0,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
struct TileMeta {
    id: TileId,
    rotation: i32,
    flipped: bool,
}

#[derive(PartialEq, Eq, Hash, Clone, Default)]
struct Tile {
    id: TileId,
    borders: [u16; 4],
    contents: Vec<Vec<char>>,
}

impl fmt::Display for TileId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TileMeta {
    fn lookup(&self, x: usize, y: usize) -> (usize, usize) {
        let mut x = x;
        let mut y = y;
        if self.flipped {
            x = S - x - 1;
        }

        for _ in 0..self.rotation {
            // Rotate anti clockwise
            let temp = S - x - 1;
            x = y;
            y = temp;
        }
        (x, y)
    }

    fn border(&self, tile: &Tile, dir: Dir) -> u16 {
        let mut dir = dir;
        let mut flip_contents = false;

        // Flipping happens after rotation
        if self.flipped {
            match dir {
                Dir::Top | Dir::Bottom => {
                    flip_contents = true;
                }
                Dir::Left => {
                    dir = Dir::Right;
                }
                Dir::Right => {
                    dir = Dir::Left;
                }
            }
        }

        let original_dir = dir.rotate(-self.rotation);

        if original_dir as u8 / 2 != dir as u8 / 2 {
            flip_contents = !flip_contents;
        }

        let border = tile.border(original_dir);
        if flip_contents {
            Tile::invert(border)
        } else {
            border
        }
    }

    fn transform(id: TileId, dir1: Dir, dir2: Dir, flip: Flipped) -> TileMeta {
        let mut rotation = Dir::rotation(&dir2, &dir1);
        let mut flipped = false;

        let contents_flipped = if dir1 as u8 / 2 != dir2 as u8 / 2 {
            Flipped::Yes
        } else {
            Flipped::No
        };

        if flip != contents_flipped {
            if dir1 == Dir::Left || dir1 == Dir::Right {
                rotation += 2;
            }
            flipped = true;
        }

        TileMeta {
            id,
            rotation: rotation % 4,
            flipped,
        }
    }
}

impl Tile {
    fn encode(line: &[char]) -> u16 {
        assert_eq!(line.len(), 10);
        line.iter()
            .map(|x| match x {
                '.' => 0 as u16,
                '#' => 1 as u16,
                _ => unreachable!(),
            })
            .fold(0, |a, x| a * 2 + x)
    }

    fn invert(x: u16) -> u16 {
        let mut inverted = 0;
        let mut x = x;
        for _ in 0..10 {
            inverted = inverted * 2 + (x & 1);
            x /= 2;
        }
        inverted
    }

    fn border(&self, dir: Dir) -> u16 {
        self.borders[dir as usize]
    }

    fn to_meta(&self) -> TileMeta {
        TileMeta {
            id: self.id,
            rotation: 0,
            flipped: false,
        }
    }
}

impl FromStr for Tile {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        let id_line = lines.next().ok_or("No tile header")?;
        let id = TileId(id_line[5..id_line.len() - 1].parse::<u64>()?);
        let contents = lines.map(|x| x.chars().collect_vec()).collect::<Vec<_>>();

        let mut borders = [0; 4];
        borders[Dir::Top as usize] = Tile::encode(&contents[0]);
        borders[Dir::Bottom as usize] = Tile::encode(&contents[contents.len() - 1]);
        borders[Dir::Left as usize] =
            Tile::encode(&contents.iter().map(|x| x[0]).collect::<Vec<_>>());
        borders[Dir::Right as usize] =
            Tile::encode(&contents.iter().map(|x| x[x.len() - 1]).collect::<Vec<_>>());

        Ok(Tile {
            id,
            contents,
            borders,
        })
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Tile: {}\n", self.id))?;

        f.write_fmt(format_args!("       {:5}      \n", self.border(Dir::Top)))?;
        f.write_fmt(format_args!(
            "      !{:5}      \n",
            Tile::invert(self.border(Dir::Top))
        ))?;
        f.write_fmt(format_args!(
            " {:5}       {:5}\n",
            self.border(Dir::Left),
            self.border(Dir::Right)
        ))?;
        f.write_fmt(format_args!(
            "!{:5}      !{:5}\n",
            Tile::invert(self.border(Dir::Left)),
            Tile::invert(self.border(Dir::Right))
        ))?;
        f.write_fmt(format_args!(
            "       {:5}      \n",
            self.border(Dir::Bottom)
        ))?;
        f.write_fmt(format_args!(
            "      !{:5}      \n",
            Tile::invert(self.border(Dir::Bottom))
        ))?;

        f.write_str("\n")
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.contents {
            for ch in line {
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Dir {
    fn from(val: i32) -> Dir {
        match ((val % 4) + 4) % 4 {
            0 => Dir::Top,
            1 => Dir::Right,
            2 => Dir::Bottom,
            3 => Dir::Left,
            _ => unreachable!(),
        }
    }

    fn rotate(&self, clockwise: i32) -> Dir {
        Dir::from(*self as i32 + clockwise)
    }

    pub fn rotation(d1: &Dir, d2: &Dir) -> i32 {
        ((*d2 as i32 - *d1 as i32) %4 + 4) % 4
    }
}

fn parse_tiles(buffer: &str) -> Result<HashMap<TileId, Tile>, Box<dyn Error>> {
    Ok(buffer
        .trim()
        .split("\n\n")
        .map(|x| x.parse::<Tile>().unwrap())
        .map(|x| (x.id, x))
        .collect::<HashMap<_, _>>())
}

fn identify_corners(tiles: &HashMap<TileId, Tile>) -> Vec<TileId> {
    let mut border_dict: HashMap<u16, Vec<&Tile>> = HashMap::new();
    for (_id, tile) in tiles.iter() {
        for border in &tile.borders {
            let entry = border_dict.entry(*border).or_insert(vec![]);
            entry.push(tile);
            let flipped_entry = border_dict.entry(Tile::invert(*border)).or_insert(vec![]);
            flipped_entry.push(tile);
        }
    }

    let mut tile_unshared_count: HashMap<TileId, usize> = HashMap::new();
    for tiles in border_dict.values() {
        if tiles.len() == 1 {
            *tile_unshared_count.entry(tiles[0].id).or_insert(0) += 1;
        }
    }

    tile_unshared_count
        .iter()
        .filter(|(_x, y)| **y == 4)
        .map(|(x, _y)| *x)
        .collect::<Vec<_>>()
}

fn print_grid(grid: &Vec<Vec<TileMeta>>, tiles: &HashMap<TileId, Tile>) {
    for row in grid {
        for y in 0..10 {
            for col in row {
                if col.id.0 != 0 {
                    for x in 0..10 {
                        let (x, y) = col.lookup(x, y);
                        print!("{}", tiles[&col.id].contents[y][x]);
                    }
                    print!(" ");
                }
            }
            println!();
        }
        println!();
    }
}

fn make_picture(grid: &Vec<Vec<TileMeta>>, tiles: &HashMap<TileId, Tile>) -> Vec<Vec<char>> {
    let mut pic = vec![];

    for row in grid {
        for y in 1..(S - 1) {
            let mut output = vec![];
            for col in row {
                if col.id.0 != 0 {
                    for x in 1..(S - 1) {
                        let (x, y) = col.lookup(x, y);
                        output.push(tiles[&col.id].contents[y][x]);
                    }
                }
            }
            pic.push(output);
        }
    }

    pic
}

// From rosetta code
fn print_picture(pic: &Vec<Vec<char>>) {
    for y in pic {
        for x in y {
            print!("\x1b[{};{}m{}\x1b[0m",
                   match x {
                       'O' => 1,
                       '#' => 5,
                       _ => 0,
                   },
                   match x {
                       'O' => 32,
                       '#' => 34,
                       _ => 0,
                   },
                   x);
        }
        println!("");
    }
}

fn solve1(buffer: &str) -> Result<u64, Box<dyn Error>> {
    let tiles = parse_tiles(buffer)?;
    Ok(identify_corners(&tiles).iter().map(|x| x.0).product())
}

fn monster_check(pic: &Vec<Vec<char>>, x: usize, y:usize) -> bool {
    for (my, row) in MONSTER.lines().enumerate() {
        for (mx, ch) in row.chars().enumerate() {
            match ch {
                '#' => if ch != pic[y + my][x + mx] { return false; }
                _ => {}
            } 
        }
    }

    true
}

fn monster_mark(pic: &mut Vec<Vec<char>>, x: usize, y: usize) {
    for (my, row) in MONSTER.lines().enumerate() {
        for (mx, ch) in row.chars().enumerate() {
            if ch == '#' {
                pic[y + my][x + mx] = 'O';
            } 
        }
    }
}

fn monster_hunt(pic: &mut Vec<Vec<char>>) -> usize {
    let pic_height = pic.len();
    let pic_width = pic[0].len();

    let monster_height = MONSTER.lines().count();
    let monster_width: usize = MONSTER.lines().next().unwrap().len();

    let mut count = 0;
    for y in 0..(pic_height - monster_height) {
        for x in 0..(pic_width - monster_width) {
            if monster_check(pic, x, y) {
                count += 1;
                monster_mark(pic, x, y);
            }
        }
    }

    count
}

fn rotate_pic(pic: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let pic_height = pic.len();
    let pic_width = pic[0].len();

    // height & width are interchanged
    let mut new_pic: Vec<Vec<char>> = vec![vec![' '; pic_height]; pic_width];

    for (y, col) in pic.iter().enumerate() {
        for (x, ch) in col.iter().enumerate() {
            let new_x = pic_height - y - 1;
            let new_y = x;
            new_pic[new_y][new_x] = *ch;
        }
    }

    new_pic
}

fn flip_pic(pic: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let pic_height = pic.len();
    let pic_width = pic[0].len();

    // height & width are interchanged
    let mut new_pic: Vec<Vec<char>> = vec![vec![' '; pic_width]; pic_height];

    for (y, col) in pic.iter().enumerate() {
        for (x, ch) in col.iter().enumerate() {
            let new_x = pic_width - x - 1;
            let new_y = y;
            new_pic[new_y][new_x] = *ch;
        }
    }

    new_pic
}

fn solve2(buffer: &str) -> Result<usize, Box<dyn Error>> {
    let tiles = parse_tiles(buffer)?;
    let mut corners = identify_corners(&tiles);
    corners.sort();

    let mut borders: HashMap<u16, Vec<(TileId, Dir, Flipped)>> = HashMap::new();
    for (id, tile) in tiles.iter() {
        for (i, border) in tile.borders.iter().enumerate() {
            let entry = borders.entry(*border).or_insert(vec![]);
            entry.push((*id, Dir::from(i as i32), Flipped::No));
            let flipped_entry = borders.entry(Tile::invert(*border)).or_insert(vec![]);
            flipped_entry.push((*id, Dir::from(i as i32), Flipped::Yes));
        }
    }

    let mut top_left = tiles[&corners[1]].to_meta(); // for testing
    let mut corner_dirs = HashSet::new();
    for (i, border) in tiles[&corners[1]].borders.iter().enumerate() {
        if borders[border].len() > 1 {
            corner_dirs.insert(Dir::from(i as i32));
        }
    }
    let mut desired_dirs = HashSet::new();
    desired_dirs.insert(Dir::Right);
    desired_dirs.insert(Dir::Bottom);

    let mut corner_rotation = 0;
    while corner_dirs != desired_dirs {
        corner_rotation += 1;
        corner_dirs = corner_dirs.into_iter()
            .map(|x| x.rotate(1))
            .collect::<HashSet<_>>();
    }
    top_left.rotation = corner_rotation;

    let total = tiles.len();
    let side = ((total as f64).sqrt()) as usize;

    let mut grid: Vec<Vec<TileMeta>> = vec![vec![Default::default(); side]; side];

    grid[0][0] = top_left;

    // Fill the first row
    for i in 1..side {
        let prev = &grid[0][i - 1];
        let constraint = prev.border(&tiles[&prev.id], Dir::Right);
        let option = *&borders[&constraint]
            .iter()
            .filter(|x| x.0 != prev.id)
            .next()
            .unwrap();

        grid[0][i] = TileMeta::transform(option.0, Dir::Left, option.1, option.2);
    }

    // Fill the rest of the grid row by row
    for y in 1..side {
        for x in 0..side {
            let prev = &grid[y - 1][x];
            let constraint = prev.border(&tiles[&prev.id], Dir::Bottom);
            let options = &borders[&constraint]
                .iter()
                .filter(|x| x.0 != prev.id)
                .copied()
                .collect::<Vec<_>>();
            assert_eq!(options.len(), 1);
            let option = options[0];
            let transform = TileMeta::transform(option.0, Dir::Top, option.1, option.2);
            grid[y][x] = transform;
        }
    }

    println!();
    print_grid(&grid, &tiles);
    let mut pic = make_picture(&grid, &tiles);

    for i in 0..8 {
        let monsters = monster_hunt(&mut pic);
        if monsters > 0 {
            print_picture(&pic);
            println!();
            let roughness = pic.iter().flatten().filter(|x| **x == '#').count();
            return Ok(roughness);
        }

        pic = rotate_pic(&pic);
        if i == 4 {
            pic = flip_pic(&pic);
        }
    }

    Ok(0)
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
    fn test_encode() {
        for test in &["##.#.#####", ".#..######", "..#.......", "######...."] {
            let mut num = test.to_string();
            num = num.replace("#", "1");
            num = num.replace(".", "0");
            let num = u16::from_str_radix(&num, 2).unwrap();
            assert_eq!(num, Tile::encode(&test.chars().collect::<Vec<_>>()));
        }
    }

    #[test]
    fn test_invert() {
        let test = u16::from_str_radix("0001010010", 2).unwrap();
        let inverted = u16::from_str_radix("0100101000", 2).unwrap();
        let x = Tile::invert(test);
        assert_eq!(inverted, x);
    }

    #[test]
    fn test_lookup() {
        let meta = TileMeta {
            flipped: false,
            rotation: 2,
            ..Default::default()
        };
        assert_eq!(meta.lookup(9, 9), (0, 0));

        let meta = TileMeta {
            flipped: false,
            rotation: 1,
            ..Default::default()
        };
        assert_eq!(meta.lookup(1, 2), (2, 8));

        let meta = TileMeta {
            flipped: true,
            rotation: 2,
            ..Default::default()
        };
        assert_eq!(meta.lookup(1, 2), (1, 7));
    }

    #[test]
    fn test_dir_lookup() {
        let tile = Tile {
            borders: [1, 2, 3, 4],
            ..Default::default()
        };
        let mut meta = TileMeta {
            flipped: false,
            rotation: 1,
            ..Default::default()
        };
        assert_eq!(
            meta.border(&tile, Dir::Top),
            Tile::invert(tile.border(Dir::Left))
        );

        meta.rotation = 2;
        assert_eq!(
            meta.border(&tile, Dir::Top),
            Tile::invert(tile.border(Dir::Bottom))
        );

        meta.rotation = 3;
        assert_eq!(meta.border(&tile, Dir::Top), tile.border(Dir::Right));
    }

    #[test]
    fn test_flipped_dir_lookup() {
        let tile = Tile {
            borders: [1, 2, 3, 4],
            ..Default::default()
        };
        let mut meta = TileMeta {
            flipped: true,
            rotation: 1,
            ..Default::default()
        };
        assert_eq!(meta.border(&tile, Dir::Top), tile.border(Dir::Left));

        meta.rotation = 2;
        assert_eq!(meta.border(&tile, Dir::Top), tile.border(Dir::Bottom));
        assert_eq!(
            meta.border(&tile, Dir::Right),
            Tile::invert(tile.border(Dir::Right))
        );

        meta.rotation = 3;
        assert_eq!(
            meta.border(&tile, Dir::Top),
            Tile::invert(tile.border(Dir::Right))
        );
    }

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT).unwrap(), 20899048083289);
    }

    #[test]
    fn test2() {
        dbg!(solve2(INPUT));
    }
}
