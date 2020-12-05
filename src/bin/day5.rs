use std::env;
use std::error::Error;
use std::io::{self, Read};

fn bsp((min, max): (u32, u32), lower: bool) -> (u32, u32) {
    if lower {
        (min, (min + max) / 2)
    } else {
        ((min + max + 1) / 2, max)
    }
}

fn seatid(entry: &str) -> u32 {
    let (row, _) = entry[..entry.len() - 3]
        .chars()
        .map(|x| x == 'F')
        .fold((0, 127), bsp);

    let (col, _) = entry[entry.len() - 3..]
        .chars()
        .map(|x| x == 'L')
        .fold((0, 7), bsp);

    row * 8 + col
}

fn solve1(buffer: &str) -> Result<String, Box<dyn Error>> {
    Ok(buffer.lines().map(seatid).max().unwrap().to_string())
}

fn solve2(buffer: &str) -> Result<String, Box<dyn Error>> {
    let mut used_seats = buffer.lines().map(seatid).collect::<Vec<u32>>();
    used_seats.sort();
    let result = used_seats
        .iter()
        .zip(&used_seats[1..])
        .find(|&(a, b)| b - a > 1)
        .unwrap();
    Ok((result.0 + 1).to_string())
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
