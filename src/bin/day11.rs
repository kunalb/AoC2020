use std::env;
use std::error::Error;
use std::io::{self, Read};
use itertools::Itertools;

fn parse(buffer: &str) -> Vec<Vec<char>> {
    buffer
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn simulate<F>(buffer: &str, f: F) -> usize
where
    F: Fn(&Vec<Vec<char>>, usize, usize) -> char,
{
    let mut grid = parse(buffer);
    let mut next_grid = grid.clone();

    loop {
        let mut changed = false;

        for row in grid.iter().enumerate() {
            for col in row.1.iter().enumerate() {
                let ch = f(&grid, row.0, col.0);
                changed |= ch != *col.1;
                next_grid[row.0][col.0] = ch;
            }
        }

        let temp = grid;
        grid = next_grid;
        next_grid = temp;

        if !changed {
            return grid.iter().flatten().filter(|x| **x == '#').count();
        }
    }
}

fn get(grid: &Vec<Vec<char>>, row: i32, col: i32) -> Option<char> {
    if row < 0 || col < 0 {
        return None;
    } else {
        grid.get(row as usize).and_then(|row| row.get(col as usize)).copied()
    }
}

fn solve1(buffer: &str) -> usize {
    simulate(buffer, |grid, row, col| {
        let occ = (-1..=1).cartesian_product(-1..=1)
            .filter(|(dr, dc)| *dr != 0 || *dc != 0)
            .filter(|(dr, dc)| match get(grid, row as i32 + dr, col as i32 + dc) {
                Some('#') => true,
                _ => false,
            })
            .count();

        match grid[row][col] {
            'L' if occ == 0 => '#',
            '#' if occ >= 4 => 'L',
            x => x,
        }
    })
}

fn solve2(buffer: &str) -> usize {
    simulate(buffer, |grid, row, col| {
        let occ = (-1..=1).cartesian_product(-1..=1)
            .filter(|(dr, dc)| *dr != 0 || *dc != 0)
            .filter(|(dr, dc)| {
                let mut r: i32 = row as i32;
                let mut c: i32 = col as i32;

                loop {
                    r += dr;
                    c += dc;

                    match get(grid, r, c) {
                        Some('#') => return true,
                        Some('L') | None => break,
                        _ => {}
                    }
                }

                false
            })
            .count();

        match grid[row][col] {
            'L' if occ == 0 => '#',
            '#' if occ >= 5 => 'L',
            x => x,
        }
    })
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
