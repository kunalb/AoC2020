use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::{self, Read};

fn parse(buffer: &str) -> Vec<u64> {
    let mut adapters = buffer
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);

    adapters
}

fn solve1(buffer: &str) -> u64 {
    let adapters = parse(buffer);
    let diffs = adapters
        .iter()
        .zip(&adapters[1..])
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();
    (diffs.iter().filter(|x| **x == 1).count() * diffs.iter().filter(|x| **x == 3).count()) as u64
}

fn ways(adapters: &Vec<u64>, pos: usize, cache: &mut HashMap<usize, u64>) -> u64 {
    if cache.contains_key(&pos) {
        return cache[&pos];
    }

    let mut total = 0;

    if pos == adapters.len() - 1 {
        return 1;
    }

    for i in 1.. {
        if pos + i >= adapters.len() ||  adapters[pos + i] - adapters[pos] > 3 {
            break;
        }

        total += ways(adapters, pos + i, cache);
    }

    cache.insert(pos, total);
    return total;
}

fn solve2(buffer: &str) -> u64 {
    let adapters = parse(buffer);
    let mut cache: HashMap<usize, u64> = HashMap::new();
    ways(&adapters, 0, &mut cache)
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

    eprintln!("Time: {}us", now.elapsed().as_micros());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {}

    #[test]
    fn test2() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(solve2(&input), 8);
    }
}
