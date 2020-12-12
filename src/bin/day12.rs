use std::env;
use std::error::Error;
use std::io::{self, Read};

fn solve1(buffer: &str) -> Result<i32, Box<dyn Error>> {
    let mut dir: (i32, i32) = (1, 0);
    let mut pos: (i32, i32) = (0, 0);

    for line in buffer.lines() {
        let inst = &line[0..1];
        let dist = &line[1..].parse::<i32>()?;

        match inst {
            "N" => {
                pos.1 += dist;
            }
            "E" => {
                pos.0 += dist;
            }
            "W" => {
                pos.0 -= dist;
            }
            "S" => {
                pos.1 -= dist;
            }
            "L" => {
                for _ in 0..(dist / 90) {
                    dir = (-dir.1, dir.0)
                }
            }
            "R" => {
                for _ in 0..(dist / 90) {
                    dir = (dir.1, -dir.0)
                }
            }
            "F" => {
                pos.0 += dir.0 * dist;
                pos.1 += dir.1 * dist;
            }
            _ => unreachable!(),
        }
    }

    Ok(pos.0.abs() + pos.1.abs())
}

fn solve2(buffer: &str) -> Result<i32, Box<dyn Error>> {
    let mut waypoint: (i32, i32) = (10, 1);
    let mut pos: (i32, i32) = (0, 0);

    for line in buffer.lines() {
        let inst = &line[0..1];
        let dist = &line[1..].parse::<i32>()?;

        match inst {
            "N" => {
                waypoint.1 += dist;
            }
            "E" => {
                waypoint.0 += dist;
            }
            "W" => {
                waypoint.0 -= dist;
            }
            "S" => {
                waypoint.1 -= dist;
            }
            "L" => {
                for _ in 0..(dist / 90) {
                    waypoint = (-waypoint.1, waypoint.0);
                }
            }
            "R" => {
                for _ in 0..(dist / 90) {
                    waypoint = (waypoint.1, -waypoint.0);
                }
            }
            "F" => {
                pos.0 += waypoint.0 * dist;
                pos.1 += waypoint.1 * dist;
            }
            _ => unreachable!(),
        }
    }

    Ok(pos.0.abs() + pos.1.abs())
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::Instant::now();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "2" {
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

    const INPUT: &'static str = "F10
N3
F7
R90
F11";

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT).unwrap(), 25);
    }
}
