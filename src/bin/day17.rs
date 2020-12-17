use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::{self, Read};

struct Neighbors {
    ranges: Vec<(i64, i64)>,
    state: Vec<i64>,
    counter: usize,
    total: usize,
    dims: usize,
}

impl Neighbors {
    fn new(root: &Vec<i64>) -> Neighbors {
        Neighbors::new_ranges(root.iter().map(|x| (x - 1, x + 1)).collect::<Vec<_>>())
    }

    fn new_ranges(ranges: Vec<(i64, i64)>) -> Neighbors {
        let dims = ranges.len();
        let total = ranges.iter().map(|(a, b)| (b - a + 1) as usize).product();
        let state = ranges.iter().map(|(a, _)| *a).collect::<Vec<_>>();

        Neighbors {
            ranges,
            dims,
            counter: 0,
            total,
            state,
        }
    }
}

impl Iterator for Neighbors {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter >= self.total {
            return None;
        }
        self.counter += 1;

        let result = Some(self.state.clone());

        for i in (0..self.dims).rev() {
            self.state[i] += 1;
            if self.state[i] > self.ranges[i].1 {
                self.state[i] = self.ranges[i].0
            } else {
                break;
            }
        }

        result
    }
}

fn print_cube(hypercube: &HashMap<Vec<i64>, bool>, ranges: &Vec<(i64, i64)>) {
    for outer_path in Neighbors::new_ranges(ranges[..ranges.len() - 2].to_vec()) {
        println!("{:?}", outer_path);
        let y_range = ranges[ranges.len() - 2];
        let x_range = ranges[ranges.len() - 1];
        for y in y_range.0..=y_range.1 {
            for x in x_range.0..=x_range.1 {
                let mut path = outer_path.clone();
                path.push(y);
                path.push(x);
                let pt = hypercube.get(&path).unwrap_or(&false);
                print!("{} ", if *pt { '#' } else { '.' });
            }
            println!();
        }

        println!();
    }
}

fn solve(buffer: &str, dims: usize) -> usize {
    assert!(dims >= 2);

    let mut hypercube: HashMap<Vec<i64>, bool> = HashMap::new();
    let mut next = hypercube.clone();

    let mut ranges: Vec<(i64, i64)> = vec![(0, 0); dims];

    for (y, line) in buffer.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let mut path = vec![0 as i64; dims];
            path[dims - 1] = x as i64;
            path[dims - 2] = y as i64;

            let pt = hypercube.entry(path).or_insert(false);
            *pt = match ch {
                '.' => false,
                '#' => true,
                _ => unreachable!(),
            };

            ranges[dims - 1].1 = x as i64;
        }

        ranges[dims - 2].1 = y as i64;
    }

    for _ in 0..6 {
        let current_ranges = ranges.iter().map(|(a, b)| (a - 1, b + 1)).collect::<Vec<_>>();
        ranges = vec![(0, 0); dims];

        for path in Neighbors::new_ranges(current_ranges) {
            let current = hypercube.get(&path).unwrap_or(&false);
            let neighbors = Neighbors::new(&path)
                .filter(|n| *n != path)
                .filter_map(|n| hypercube.get(&n))
                .filter(|a| **a)
                .count();

            let next_state = match (current, neighbors) {
                (true, 2) | (_, 3) => true,
                _ => false,
            };

            if next_state {
                for i in 0..dims {
                    if path[i] > ranges[i].1 {
                        ranges[i].1 = path[i];
                    } else if path[i] < ranges[i].0 {
                        ranges[i].0 = path[i];
                    }
                }
            }

            next.insert(path, next_state);
        }

        let temp = hypercube;
        hypercube = next;
        next = temp;
        // print_cube(&hypercube, &ranges);
    }

    hypercube.values().filter(|a| **a).count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::Instant::now();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "2" {
        println!("{}", solve(&buffer, 4));
    } else {
        println!("{}", solve(&buffer, 3));
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
        assert_eq!(solve(INPUT, 3), 112);
    }

    #[test]
    fn test2() {
        assert_eq!(solve(INPUT, 4), 848);
    }
}
