use std::env;
use std::error::Error;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

fn parse(line: &str) -> Result<(usize, usize, u8, &[u8]), Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"(?P<min>\d+)-(?P<max>\d+) (?P<ch>.): (?P<pw>.+)"#).unwrap();
    }
    let captures = RE.captures(line).unwrap();
    Ok((captures.name("min").unwrap().as_str().parse::<usize>()?,
        captures.name("max").unwrap().as_str().parse::<usize>()?,
        captures.name("ch").unwrap().as_str().as_bytes()[0],
        captures.name("pw").unwrap().as_str().as_bytes()))
}

fn solve1(buffer: &str) -> Result<String, Box<dyn Error>> {
    let result = buffer.lines()
        .map(|line| parse(line).unwrap())
        .filter(|(min, max, ch, bytes)| {
            let count = bytes.iter().filter(|b| *b == ch).count();
            count >= *min && count <= *max
        })
        .count();
    Ok(format!("{}",result)) 
}

fn solve2(buffer: &str) -> Result<String, Box<dyn Error>> {
    let result = buffer.lines()
        .map(|line| parse(line).unwrap())
        .filter(|(min, max, ch, bytes)| 
            (bytes[(min - 1) as usize] == *ch) ^ (bytes[(max - 1) as usize] == *ch)
        )
        .count();
    Ok(format!("{}",result)) 
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "2" {
        println!("{}", solve2(&buffer)?);
    } else {
        println!("{}", solve1(&buffer)?);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(solve1(input).unwrap(), "2");
    }

    #[test]
    fn test2() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(solve2(input).unwrap(), "1");
    }
}
