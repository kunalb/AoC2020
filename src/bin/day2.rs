use std::io::{self, Read};
use std::error::Error;
use std::env;

fn solve1(buffer: &str) -> Result<String, Box<dyn Error>> {
    let mut good = 0;
    for line in buffer.lines() {
        let parts = line.split(":").collect::<Vec<_>>();
        let bytes = parts[1].trim().as_bytes();

        let input0 = parts[0].split(" ").collect::<Vec<_>>();
        let range = input0[0].split("-").collect::<Vec<_>>();
        let ch = input0[1].as_bytes()[0];

        let min = range[0].parse::<u32>()?;
        let max = range[1].parse::<u32>()?;
        
        let mut count = 0;
        for b in bytes {
            if *b == ch {
                count += 1;
            }
        }

        if count >= min && count <= max {
            good += 1;
        }

    }

    Ok(format!("{}", good))
}

fn solve2(buffer: &str) -> Result<String, Box<dyn Error>> {
    let mut good = 0;
    for line in buffer.lines() {
        let parts = line.split(":").collect::<Vec<_>>();
        let bytes = parts[1].trim().as_bytes();

        let input0 = parts[0].split(" ").collect::<Vec<_>>();
        let range = input0[0].split("-").collect::<Vec<_>>();
        let ch = input0[1].as_bytes()[0];

        let min = range[0].parse::<u32>()?;
        let max = range[1].parse::<u32>()?;

        if (bytes[(min - 1) as usize] == ch) ^ (bytes[(max - 1) as usize] == ch) {
            good += 1;
        }
    }

    Ok(format!("{}", good))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

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
        let input="1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        println!("{}", solve1(input).unwrap());
    }

    #[test]
    fn test2() {
        let input="1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        println!("{}", solve2(input).unwrap());
    }
}
