use std::io::{self, Read};
use std::error::Error;
use std::env;
use std::collections::HashMap;

fn solve(buffer: &str, limit: usize) -> Result<usize, Box<dyn Error>> {
    let numbers = buffer.trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut last_turn: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut last_number = 0;

    for turn in 1..=limit {
        if turn <= numbers.len() {
            last_number = numbers[turn - 1];
            last_turn.insert(last_number, (turn, turn));
        } else {
            let next_number = if last_turn.contains_key(&last_number) {
                last_turn[&last_number].1 - last_turn[&last_number].0
            } else {
                0
            };
            if last_turn.contains_key(&next_number) {
                last_turn.get_mut(&next_number).unwrap().0 = last_turn[&next_number].1;
                last_turn.get_mut(&next_number).unwrap().1 = turn;
            } else {
                last_turn.insert(next_number, (turn, turn));
            }
            last_number = next_number;
        }

        // eprintln!("Turn {}: {}", turn, last_number);
    }

    Ok(last_number)
}

fn solve1(buffer: &str) -> Result<usize, Box<dyn Error>> {
    solve(buffer, 2020)
}

fn solve2(buffer: &str) -> Result<usize, Box<dyn Error>> {
    solve(buffer, 30000000)
}


fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::Instant::now();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    if args.len() >1 && args[1] == "2" {
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
    fn test() {
        assert_eq!(solve1("0,3,6").unwrap(), 436);
    }

    #[test]
    fn test_recurrence() {
    }
}
