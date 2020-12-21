use std::io::{self, Read};
use std::error::Error;
use std::env;

use lazy_static::lazy_static;
use regex::Regex;

fn parse(line: &str) -> Result<&str, Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(?P<cap>.*)"#).unwrap();
    }
    let captures = RE.captures(line).unwrap();
    let result = captures.name("cap").unwrap().as_str();
    Ok(result)
}

fn solve1(buffer: &str) -> Result<String, Box<dyn Error>> {
    Ok(format!("Solution 1: {}", parse(buffer)?))
}

fn solve2(buffer: &str) -> Result<String, Box<dyn Error>> {
    Ok(format!("Solution 2: {}", parse(buffer)?))
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
    fn test1() {
    }

    #[test]
    fn test2() {
    }
}
