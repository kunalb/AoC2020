use std::io::{self, Read};
use std::error::Error;

const MOD: u64 = 20201227;

fn mod_exp(base: u64, e: u64, m: u64) -> u64 {
    let mut result = 1;
    let mut e = e;
    let mut base = base;

    while e > 0 {
        if e & 1 > 0 {
            result = (result * base) % m;
        }

        base = (base * base) % m;
        e >>= 1;
    }

    result
}

fn solve(buffer: &str) -> Result<u64, Box<dyn Error>> {
    let mut lines = buffer.lines();
    let card_public_key = lines.next().unwrap().parse::<u64>()?;
    let door_public_key = lines.next().unwrap().parse::<u64>()?;

    let mut loop_size = 1;
    loop {
        let result = mod_exp(7, loop_size, MOD); 

        if result == card_public_key {
            return Ok(mod_exp(door_public_key, loop_size, MOD))
        }

        if result == door_public_key {
            return Ok(mod_exp(card_public_key, loop_size, MOD))
        }

        loop_size += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::Instant::now();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{}", solve(&buffer)?);

    eprintln!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve("5764801\n17807724").unwrap(), 14897079);
    }
}
