use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::{self, Read};
use itertools::iproduct;

fn solve1(buffer: &str) -> Result<usize, Box<dyn Error>> {
    let mut cube: HashMap<i64, HashMap<i64, HashMap<i64, bool>>> = HashMap::new();
    let mut next_cube = cube.clone();

    let mut z_range: (i64, i64) = (0, 0);
    let mut x_range: (i64, i64) = (0, 0);
    let mut y_range: (i64, i64) = (0, 0);

    for (y, line) in buffer.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pt = cube
                .entry(0)
                .or_insert(HashMap::new())
                .entry(y as i64)
                .or_insert(HashMap::new())
                .entry(x as i64)
                .or_insert(false);
            *pt = match ch {
                '.' => false,
                '#' => true,
                _ => unreachable!(),
            };

            x_range.1 = x as i64;
        }

        y_range.1 = y as i64;
    }

    for cycle in 0..6 {
        for z in (z_range.0 - 1)..=(z_range.1 + 1) {
            for y in (y_range.0 - 1)..=(y_range.1 + 1) {
                for x in (x_range.0 - 1)..=(x_range.1 + 1) {
                    let pt = next_cube
                        .entry(z)
                        .or_insert(HashMap::new())
                        .entry(y)
                        .or_insert(HashMap::new())
                        .entry(x)
                        .or_insert(false);

                    let current = cube
                        .get(&z)
                        .and_then(|ys| ys.get(&y))
                        .and_then(|xs| xs.get(&x))
                        .unwrap_or(&false);

                    let neighbors = iproduct!(z - 1..=z + 1, y - 1..=y + 1, x - 1..=x + 1)
                        .filter(|(zz, yy, xx)| *zz != z || *yy != y || *xx != x)
                        .filter_map(|(zz, yy, xx)| {
                            cube.get(&zz)
                                .and_then(|ys| ys.get(&yy))
                                .and_then(|xs| xs.get(&xx))
                        })
                        .filter(|a| **a)
                        .count();

                    *pt = match (current, neighbors) {
                        (true, 2) | (true, 3) => true,
                        (false, 3) => true,
                        _ => false,
                    };

                    if *pt {
                        z_range.0 = if z_range.0 > z { z } else { z_range.0 };
                        z_range.1 = if z_range.1 < z { z } else { z_range.1 };
                        y_range.0 = if y_range.0 > y { y } else { y_range.0 };
                        y_range.1 = if y_range.1 < y { y } else { y_range.1 };
                        x_range.0 = if x_range.0 > x { x } else { x_range.0 };
                        x_range.1 = if x_range.1 < x { x } else { x_range.1 };
                    }
                }
            }
        }

        let temp = cube;
        cube = next_cube;
        next_cube = temp;
    }

    Ok(cube
        .values()
        .map(|ys| ys.values())
        .flatten()
        .map(|xs| xs.values())
        .flatten()
        .filter(|a| **a)
        .count())
}

fn solve2(buffer: &str) -> Result<usize, Box<dyn Error>> {
    let mut cube: HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, bool>>>> = HashMap::new();
    let mut next_cube = cube.clone();

    let mut w_range: (i64, i64) = (0, 0);
    let mut z_range: (i64, i64) = (0, 0);
    let mut x_range: (i64, i64) = (0, 0);
    let mut y_range: (i64, i64) = (0, 0);

    for (y, line) in buffer.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pt = cube
                .entry(0)
                .or_insert(HashMap::new())
                .entry(0)
                .or_insert(HashMap::new())
                .entry(y as i64)
                .or_insert(HashMap::new())
                .entry(x as i64)
                .or_insert(false);
            *pt = match ch {
                '.' => false,
                '#' => true,
                _ => unreachable!(),
            };

            x_range.1 = x as i64;
        }

        y_range.1 = y as i64;
    }

    for cycle in 0..6 {
        let w_range2 = w_range.clone();
        let z_range2 = z_range.clone();
        let y_range2 = y_range.clone();
        let x_range2 = x_range.clone();

        w_range = (0, 0);
        z_range = (0, 0);
        y_range = (0, 0);
        x_range = (0, 0);

        for w in (w_range2.0 - 1)..=(w_range2.1 + 1) {
            for z in (z_range2.0 - 1)..=(z_range2.1 + 1) {
                for y in (y_range2.0 - 1)..=(y_range2.1 + 1) {
                    for x in (x_range2.0 - 1)..=(x_range2.1 + 1) {
                        let pt = next_cube
                            .entry(w)
                            .or_insert(HashMap::new())
                            .entry(z)
                            .or_insert(HashMap::new())
                            .entry(y)
                            .or_insert(HashMap::new())
                            .entry(x)
                            .or_insert(false);

                        let current = cube
                            .get(&w)
                            .and_then(|zs| zs.get(&z))
                            .and_then(|ys| ys.get(&y))
                            .and_then(|xs| xs.get(&x))
                            .unwrap_or(&false);

                        let neighbors = iproduct!(w - 1..=w + 1, z - 1..=z + 1, y - 1..=y + 1, x - 1..=x + 1)
                                .filter(|(ww, zz, yy, xx)| {
                                    *ww != w || *zz != z || *yy != y || *xx != x
                                })
                                .filter_map(|(ww, zz, yy, xx)| {
                                    cube.get(&ww)
                                        .and_then(|zs| zs.get(&zz))
                                        .and_then(|ys| ys.get(&yy))
                                        .and_then(|xs| xs.get(&xx))
                                })
                                .filter(|a| **a)
                                .count();

                        *pt = match (current, neighbors) {
                            (true, 2) | (true, 3) => true,
                            (false, 3) => true,
                            _ => false,
                        };

                        if *pt {
                            w_range.0 = if w_range.0 > w { w } else { w_range.0 };
                            w_range.1 = if w_range.1 < w { w } else { w_range.1 };
                            z_range.0 = if z_range.0 > z { z } else { z_range.0 };
                            z_range.1 = if z_range.1 < z { z } else { z_range.1 };
                            y_range.0 = if y_range.0 > y { y } else { y_range.0 };
                            y_range.1 = if y_range.1 < y { y } else { y_range.1 };
                            x_range.0 = if x_range.0 > x { x } else { x_range.0 };
                            x_range.1 = if x_range.1 < x { x } else { x_range.1 };
                        }
                    }
                }
            }
        }
        let temp = cube;
        cube = next_cube;
        next_cube = temp;
    }

    Ok(cube
        .values()
        .map(|zs| zs.values())
        .flatten()
        .map(|ys| ys.values())
        .flatten()
        .map(|xs| xs.values())
        .flatten()
        .filter(|a| **a)
        .count())
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

    const INPUT: &'static str = ".#.
..#
###";

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT).unwrap(), 112);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT).unwrap(), 848);
    }
}
