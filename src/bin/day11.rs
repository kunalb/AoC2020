use std::env;
use std::error::Error;
use std::io::{self, Read};

fn parse(buffer: &str) -> Vec<Vec<char>> {
    buffer
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn solve1(buffer: &str) -> usize {
    let mut grid = parse(buffer);

    loop {
        let mut changes = 0;
        let mut nextgrid: Vec<Vec<char>> = Vec::new();

        for row in grid.iter().enumerate() {
            let mut new_row = Vec::new();
            for col in row.1.iter().enumerate() {
                let mut occ = 0;

                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }

                        let r: i32 = row.0 as i32 + i;
                        let c: i32 = col.0 as i32 + j;

                        if r >= 0
                            && c >= 0
                            && (r as usize) < grid.len()
                            && (c as usize) < row.1.len()
                            && grid[r as usize][c as usize] == '#'
                        {
                            occ += 1;
                        }
                    }
                }

                new_row.push(match *col.1 as char {
                    'L' if occ == 0 => {
                        changes += 1;
                        '#'
                    }
                    '#' if occ >= 4 => {
                        changes += 1;
                        'L'
                    }
                    x => x,
                });
            }
            nextgrid.push(new_row);
        }

        grid = nextgrid;
        if changes == 0 {
            return grid.iter().flatten().filter(|x| **x == '#').count();
        }
    }
}

fn solve2(buffer: &str) -> usize {
    let mut grid = parse(buffer);

    loop {
        let mut changes = 0;
        let mut nextgrid: Vec<Vec<char>> = Vec::new();

        for row in grid.iter().enumerate() {
            let mut new_row = Vec::new();
            for col in row.1.iter().enumerate() {
                let mut occ = 0;

                for dr in &[-1, 0, 1] {
                    for dc in &[-1, 0, 1] {
                        if *dr == 0 && *dc == 0 {
                            continue;
                        }

                        let mut r: i32 = row.0 as i32;
                        let mut c: i32 = col.0 as i32;

                        loop {
                            r += dr;
                            c += dc;

                            if r >= 0
                                && c >= 0
                                && (r as usize) < grid.len()
                                && (c as usize) < row.1.len()
                            {
                                match grid[r as usize][c as usize] {
                                    'L' => {
                                        break;
                                    }
                                    '#' => {
                                        occ += 1;
                                        break;
                                    }
                                    _ => {}
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }

                new_row.push(match *col.1 as char {
                    'L' if occ == 0 => {
                        changes += 1;
                        '#'
                    }
                    '#' if occ >= 5 => {
                        changes += 1;
                        'L'
                    }
                    x => x,
                });
            }
            nextgrid.push(new_row);
        }

        grid = nextgrid;

        if changes == 0 {
            return grid.iter().flatten().filter(|x| **x == '#').count();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::Instant::now();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "2" {
        println!("{}", solve2(&buffer));
    } else {
        println!("{}", solve1(&buffer));
    }

    eprintln!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 37);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 26);
    }
}
