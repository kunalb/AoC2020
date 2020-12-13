use std::env;
use std::error::Error;
use std::io::{self, Read};

fn solve1(buffer: &str) -> Result<u64, Box<dyn Error>> {
    let input = buffer.lines().collect::<Vec<_>>();
    let start_time = input[0].parse::<u64>()?;
    let buses = input[1]
        .split(",")
        .filter(|x| *x != "x")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut result = 0;
    let mut min = 0;
    for bus in buses {
        let min_time = ((start_time as f64 / bus as f64).ceil()) as u64 * bus;
        let delay = min_time - start_time;

        if min == 0 || min_time < min {
            result = delay * bus;
            min = min_time;
        }
    }

    Ok(result)
}

/// Multiplicative of b mod a
fn inv(b: i64, a: i64) -> i64 {
    let mut a = a;
    let mut b = if b > a { b % a } else { b };

    let orig_a = a;

    let mut rn_2 = (1, 0);
    let mut rn_1 = (0, 1);
    let mut rn = (0, 0);

    loop {
        let q = a / b;
        let r = a % b;

        rn.0 = rn_2.0 - q * rn_1.0;
        rn.1 = rn_2.1 - q * rn_1.1;

        if r == 1 {
            return if rn.1 < 0 { rn.1 + orig_a } else { rn.1 };
        }

        a = b;
        b = r;

        rn_2 = rn_1;
        rn_1 = rn;
    }
}

fn mymod(a: i64, b: i64) -> i64 {
    let result = a % b;
    if result < 0 {
        b + result
    } else {
        result
    }
}

fn solve2(buffer: &str) -> Result<i64, Box<dyn Error>> {
    let input = buffer.lines().collect::<Vec<_>>();

    let buses = input[1]
        .split(",")
        .enumerate()
        .filter(|(_, b)| b != &"x")
        .map(|(a, b)| (-1 * (a as i64), b.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    let mut base: Vec<(i64, i64)> = Vec::new();
    let mut constraints = buses;

    while constraints.len() > 1 {
        let top = constraints[0];
        base.push(constraints[0]);

        let mut next_constraints: Vec<(i64, i64)> = Vec::new();
        for constraint in &constraints[1..] {
            next_constraints.push((
                mymod(
                    inv(top.1, constraint.1) * (constraint.0 - top.0),
                    constraint.1,
                ),
                constraint.1,
            ))
        }

        constraints = next_constraints;
    }
    base.push(constraints[0]);

    let mut min_val = 0.0;
    for val in base.iter() {
        min_val = (min_val - val.0 as f64) / val.1 as f64
    }

    let mut result = min_val.ceil() as i64;
    for val in base.iter().rev() {
        result = val.1 * result + val.0
    }

    Ok(result)
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

    const INPUT: &'static str = "939
17,x,13,19";

    #[test]
    fn test1() {}

    #[test]
    fn test_inv() {
        assert_eq!(inv(17, 19), 9);
        assert_eq!(inv(13, 19), 3);
        assert_eq!(inv(17, 13), 10);
    }

    #[test]
    fn test2() {
        println!("{}", solve2(INPUT).unwrap());
    }

    #[test]
    fn test3() {
        let input = "x\n67,7,59,61";
        assert_eq!(solve2(input).unwrap(), 754018);
    }
}
