use std::io::{self, Read};
use std::error::Error;
use std::env;

fn solve1(buffer: &str) -> Result<String, Box<dyn Error>> {
    Ok(String::from("Solution 1"))
}

fn solve2(buffer: &str) -> Result<String, Box<dyn Error>> {
    Ok(String::from("Solution 2"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("So it begins!");

    let args: Vec<String> = env::args().collect();
    if args.len() >1 && args[1] == "2" {
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
    }

    #[test]
    fn test2() {
    }
}
