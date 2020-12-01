use std::io::{self, Read};
use std::error::Error;
use std::collections::HashSet;

fn solve1(nums: &[i64]) -> i64 {
    let mut set: HashSet<i64> = HashSet::new();
    set.extend(nums);

    for a in nums {
        if set.contains(&(2020 - a)) {
            return a * (2020 - a);
        }
    }

    panic!("Solution 1: Couldn't find valid numbers!");
}

fn solve2(nums: &[i64]) -> i64{
    let mut set: HashSet<i64> = HashSet::new();
    set.extend(nums);

    for a in nums {
        for b in nums {
            if set.contains(&(2020 - a - b)) {
                return a * b * (2020 - a - b);
            }
        }
    }

    panic!("Solution 2: Couldn't find valid numbers!");
}

fn parse(buffer: &str) -> Vec<i64> {
    buffer.lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let nums = parse(&buffer);
    println!("1: {}\n2: {}", solve1(&nums), solve2(&nums));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1721
979
366
299
675
1456";

    #[test]
    fn test1() {
        assert_eq!(solve1(&parse(INPUT)), 514579);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(&parse(INPUT)), 241861950);
    }
}
