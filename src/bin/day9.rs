use std::env;
use std::error::Error;
use std::io::{self, Read};

fn parse(buffer: &str) -> Vec<u64> {
    buffer
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn solve1(numbers: &[u64], pred: usize) -> u64 {
    'outer: for (i, number) in numbers[pred..].iter().enumerate() {
        let pred = &numbers[i..(i + pred)];
        for a in pred {
            for b in pred {
                if *number == a + b {
                    continue 'outer;
                }
            }
        }

        return *number;
    }
    panic!("No soln");
}

fn solve2(numbers: &[u64], pred: usize) -> u64 {
    let target = solve1(numbers, pred);

    let mut start = 0;
    let mut stop = 1;
    let mut sum = numbers[start];
    loop {
        if sum == target {
            return numbers[start..stop].iter().min().unwrap()
                + numbers[start..stop].iter().max().unwrap();
        }

        while sum + numbers[stop] > target {
            sum -= numbers[start];
            start += 1;
        }

        while sum + numbers[stop] <= target {
            sum += numbers[stop];
            stop += 1;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::Instant::now();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let numbers = parse(&buffer);

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "2" {
        println!("{}", solve2(&numbers, 25));
    } else {
        println!("{}", solve1(&numbers, 25));
    }

    eprintln!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(solve2(&parse(input), 5), 62);
    }
}
