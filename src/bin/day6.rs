use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::io::{self, Read};

fn process<F>(buffer: &str, f: F) -> usize
where
    F: Fn(&str) -> usize,
{
    buffer.split("\n\n").map(f).fold(0, |a, b| a + b)
}

fn solve1(buffer: &str) -> usize {
    process(buffer, |x| {
        x.chars()
            .filter(|&x| x != '\n')
            .collect::<HashSet<char>>()
            .len()
    })
}

fn solve2(buffer: &str) -> usize {
    process(buffer, |x| {
        x.lines()
            .map(|x| x.chars().collect::<HashSet<char>>())
            .fold(('a'..='z').collect::<HashSet<char>>(), |x, y| {
                x.intersection(&y).copied().collect::<HashSet<char>>()
            })
            .len()
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "2" {
        println!("{}", solve2(&buffer));
    } else {
        println!("{}", solve1(&buffer));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(solve2(&input), 6);
    }
}
