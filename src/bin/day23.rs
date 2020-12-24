use std::env;
use std::error::Error;
use std::io::{self, Read};

fn solve1(buffer: &str) -> Result<String, Box<dyn Error>> {
    let cups = solve(buffer, 100, 9)?;

    let mut result = String::from("");
    let mut point = 1;
    loop {
        point = cups[point];

        if point == 1 {
            break;
        }

        result += &point.to_string();
    }

    Ok(result)
}

fn solve2(buffer: &str) -> Result<usize, Box<dyn Error>> {
    let cups = solve(buffer, 10_000_000, 1_000_000)?;
    Ok(cups[1] * cups[cups[1]])
}

fn print_debug(cups: &Vec<usize>) {
    let start = cups[0];
    let mut point = start;

    let mut max_iter = cups.len();

    loop {
        print!("{} ", point);
        point = cups[point];

        max_iter -= 1;

        if point == start {
            break;
        }

        if max_iter == 0 {
            println!("!");
            print!("{:?}", cups);
            break;
        }
    }

    if max_iter > 1 {
        println!("!");
        print!("{} {:?}", max_iter, cups);
    }

    println!();
}

fn solve(buffer: &str, iter: usize, len: usize) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut cups: Vec<usize> = vec![0; len + 1];

    let input = buffer
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    // println!("{:?}", input);

    let mut prev = 0;
    for (i, &x) in input.iter().enumerate() {
        cups[prev] = x;
        prev = x;
    }

    if input.len() < len {
        cups[prev] = input.len() + 1; // Connect the two

        for i in input.len() + 1..=len {
            cups[i] = i + 1;
            prev = i;
        }
    }

    cups[prev] = input[0]; // Make it a circle

    let mut next3 = [0; 3];
    let mut point = cups[0];

    // print_debug(&cups);

    for _ in 0..iter {
        let mut t = cups[point];
        for i in 0..3 {
            next3[i] = t;
            t = cups[t];
        }

        let mut dest = if point - 1 == 0 { len } else { point - 1 };
        while next3.iter().find(|&&x| x == dest).is_some() {
            dest -= 1;
            if dest == 0 {
                dest = len;
            }
        }
        // print!("{} | ", dest);

        // Point -> 0 -> 1 -> 2 -> x ... -> Dest -> y

        // Point => x
        cups[point] = cups[next3[2]];

        // Dest => 0 -> 1 -> 2 => y
        let y = cups[dest];
        cups[dest] = next3[0];
        cups[next3[2]] = y;

        point = cups[point];
        // print_debug(&cups);
    }

    Ok(cups)
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

    #[test]
    fn test1() {
        assert_eq!(solve1("389125467").unwrap(), "67384529");
    }

    #[test]
    fn test2() {
        let results = solve("389125467", 10_000_000, 1_000_000).unwrap();
        assert_eq!(results[1] * results[results[1]], 149245887792);
    }
}
