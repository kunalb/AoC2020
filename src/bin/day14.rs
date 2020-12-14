use std::io::{self, Read};
use std::error::Error;
use std::env;
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

fn parse(line: &str) -> Result<(u64, u64), Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"mem\[(?P<loc>\d+)\] = (?P<val>\d+)"#).unwrap();
    }
    let captures = RE.captures(line.trim()).unwrap();
    Ok((captures.name("loc").unwrap().as_str().parse::<u64>()?, 
        captures.name("val").unwrap().as_str().parse::<u64>()?))
}

fn solve1(buffer: &str) -> Result<u64, Box<dyn Error>> {
    let lines = buffer.lines().collect::<Vec<_>>();

    let mut mask;
    let mut mask_ones: u64 = 0;
    let mut mask_zeros: u64 = 0;
    let mut stmt;

    let mut mem: HashMap::<u64, u64> = HashMap::new();

    for line in lines {
        if line.contains("mask") {
            mask = line.split("=").collect::<Vec<_>>()[1].trim();

            mask_ones = 0;
            mask_zeros = 0;

            for ch in mask.chars() {
                mask_ones <<= 1;
                mask_zeros <<= 1;
                match ch {
                    '1' => {
                        mask_ones += 1;
                        mask_zeros += 1;
                    }
                    '0' => {}
                    'X' => {
                        mask_zeros += 1;
                    }
                    _ => unreachable!()
                }
            }
        } else {
            stmt = parse(line)?;
            mem.insert(stmt.0, (stmt.1 & mask_zeros) | mask_ones);
        }
    }

    Ok(mem.values().sum::<u64>())
}

fn solve2(buffer: &str) -> Result<u64, Box<dyn Error>> {
    let lines = buffer.lines().collect::<Vec<_>>();

    let mut mask;
    let mut mask_ones: u64 = 0;
    let mut mask_floaters: Vec<u64> = Vec::new();
    let mut stmt;

    let mut mem: HashMap::<u64, u64> = HashMap::new();

    for line in lines {
        if line.contains("mask") {
            mask = line.split("=").collect::<Vec<_>>()[1].trim();

            mask_ones = 0;
            mask_floaters.clear();

            for (i, ch) in mask.chars().enumerate() {
                mask_ones <<= 1;
                match ch {
                    '1' => {
                        mask_ones += 1;
                    }
                    '0' => {
                    }
                    'X' => {
                        let val = 1 << (mask.len() - i - 1);
                        mask_floaters.push(val);
                    }
                    _ => unreachable!()
                }
            }
        } else {
            stmt = parse(line)?;
            for i in 0..(2 << mask_floaters.len()) {
                let mut masked_mem = stmt.0 | mask_ones;
                for (j, floater) in mask_floaters.iter().enumerate() {
                    if i & (2 << j) > 0 {
                        masked_mem |= floater;
                    } else {
                        masked_mem &= !floater;
                    }

                    mem.insert(masked_mem, stmt.1);
                }
            }
        }
    }

    Ok(mem.values().sum::<u64>())
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
