use std::env;
use std::error::Error;
use std::io::{self, Read};

fn solve(buffer: &str, limit: usize) -> Result<usize, Box<dyn Error>> {
    let numbers = buffer
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut last_turn: Vec<(usize, usize)> = vec![(0, 0); limit];

    let mut last_number = 0;

    for turn in 1..=limit {
        if turn <= numbers.len() {
            last_number = numbers[turn - 1];
            last_turn[last_number] = (turn, turn);
        } else {
            let prev_turn = last_turn[last_number];
            let next_number = prev_turn.1 - prev_turn.0;

            if let Some(next_turn) = last_turn.get_mut(next_number) {
                next_turn.0 = if next_turn == &(0, 0) {
                    turn
                } else {
                    next_turn.1
                };
                next_turn.1 = turn;
            }

            last_number = next_number;
        }
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

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "profile" {
        let buffer = String::from("0");
        println!("{}", solve(&buffer, 1000000)?);
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;

        if args.len() > 1 && args[1] == "2" {
            println!("{}", solve2(&buffer)?);
        } else {
            println!("{}", solve1(&buffer)?);
        }

        eprintln!("Time: {}ms", now.elapsed().as_millis());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1("0,3,6").unwrap(), 436);
    }
}
