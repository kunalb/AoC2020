use std::env;
use std::error::Error;
use std::io::{self, Read};

fn solve(buffer: &str, dx: &[usize], dy: &[usize]) -> Result<u64, Box<dyn Error>> {
    let mut result: u64 = 1;
    let grid = buffer
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let len = grid[0].len();

    for (dx, dy) in dx.iter().zip(dy) {
        let mut x = 0;
        let mut y = 0;
        let mut inner_result = 0;

        while y + dy < grid.len() {
            x = (x + dx) % len;
            y = y + dy;

            inner_result += if grid[y][x] == '#' as u8 { 1 } else { 0 }
        }

        result *= inner_result;
    }

    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "2" {
        println!("{}", solve(&buffer, &[1, 3, 5, 7, 1], &[1, 1, 1, 1, 2])?);
    } else {
        println!("{}", solve(&buffer, &[3], &[1])?);
    }

    Ok(())
}
