use std::env;
use std::error::Error;
use std::io::{self, Read};

fn read_program(buffer: &str) -> Vec<(&str, i32)> {
    buffer
        .lines()
        .map(|x| {
            let pieces = x.split(" ").collect::<Vec<_>>();
            (pieces[0], pieces[1].parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>()
}

fn solve1(buffer: &str) -> i32 {
    let program = read_program(buffer);

    let mut acc = 0;
    let mut pos = 0;
    let mut executed = vec![false; program.len()];

    loop {
        if executed[pos] {
            return acc;
        }
        executed[pos] = true;

        let current = program[pos];
        match current.0 {
            "acc" => {
                acc += current.1;
                pos += 1;
            }
            "jmp" => {
                pos = (pos as i32 + current.1) as usize;
            }
            "nop" => {
                pos += 1;
            }
            _ => unreachable!(),
        }
    }
}

fn solve2(buffer: &str) -> i32 {
    let program = read_program(buffer);

    for i in 0..program.len() {
        let mut acc = 0;
        let mut pos = 0;
        let mut executed = vec![false; program.len()];

        loop {
            if pos == program.len() {
                return acc;
            }

            if executed[pos] {
                break;
            }
            executed[pos] = true;

            let current = program[pos];
            match (current.0, pos == i) {
                ("acc", _) => {
                    acc += current.1;
                    pos += 1;
                }
                ("jmp", false) | ("nop", true) => {
                    pos = (pos as i32 + current.1) as usize;
                }
                ("nop", false) | ("jmp", true) => {
                    pos += 1;
                }
                _ => unreachable!(),
            }
        }
    }

    panic!("No solution");
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::Instant::now();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "2" {
        println!("{}", solve2(&buffer));
    } else {
        println!("{}", solve1(&buffer));
    }

    eprintln!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}
