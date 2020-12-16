use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

fn parse(line: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#".*?(?P<a>\d+)-(?P<b>\d+) or (?P<c>\d+)-(?P<d>\d+).*"#).unwrap();
    }
    let captures = RE.captures(line).unwrap();
    let result = vec![
        (
            captures.name("a").unwrap().as_str().parse::<usize>()?,
            captures.name("b").unwrap().as_str().parse::<usize>()?,
        ),
        (
            captures.name("c").unwrap().as_str().parse::<usize>()?,
            captures.name("d").unwrap().as_str().parse::<usize>()?,
        ),
    ];

    Ok(result)
}

fn parse2(line: &str) -> Result<(String, Vec<(usize, usize)>), Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"(?P<label>.*?): (?P<a>\d+)-(?P<b>\d+) or (?P<c>\d+)-(?P<d>\d+).*"#)
                .unwrap();
    }
    let captures = RE.captures(line).unwrap();
    let result = vec![
        (
            captures.name("a").unwrap().as_str().parse::<usize>()?,
            captures.name("b").unwrap().as_str().parse::<usize>()?,
        ),
        (
            captures.name("c").unwrap().as_str().parse::<usize>()?,
            captures.name("d").unwrap().as_str().parse::<usize>()?,
        ),
    ];

    Ok((captures.name("label").unwrap().as_str().to_string(), result))
}

fn solve1(buffer: &str) -> Result<String, Box<dyn Error>> {
    let pieces = buffer.split("\n\n").collect::<Vec<_>>();

    let mut rules: Vec<Vec<(usize, usize)>> = vec![];

    for line in pieces[0].lines() {
        if let Ok(r) = parse(line) {
            rules.push(r)
        }
    }

    let mut error_rate: usize = 0;
    // dbg!(&rules);
    'ticket: for line in pieces[2].lines().skip(1) {
        for var in line.trim().split(",") {
            let var = var.parse::<usize>()?;
            let mut passed = false;
            'rules: for rule in &rules {
                for range in rule {
                    if var <= range.1 && var >= range.0 {
                        // dbg!(var, range);
                        passed = true;
                        break 'rules;
                    }
                }
            }
            if !passed {
                // dbg!(var);
                error_rate += var;
            }
        }
    }

    Ok(error_rate.to_string())
}

fn solve2(buffer: &str) -> Result<String, Box<dyn Error>> {
    let pieces = buffer.split("\n\n").collect::<Vec<_>>();

    let mut rules: Vec<(String, Vec<(usize, usize)>)> = vec![];
    let mut positions: HashMap<String, HashSet<usize>> = HashMap::new();

    let myticket = pieces[1]
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for line in pieces[0].lines() {
        if let Ok(r) = parse2(line) {
            positions.insert(r.0.clone(), (0..myticket.len()).collect::<HashSet<usize>>());
            rules.push(r);
        }
    }

    for line in pieces[2].lines().skip(1) {
        for (i, var) in line.trim().split(",").enumerate() {
            let var = var.parse::<usize>()?;
            let mut passed = false;
            'rules: for rule in &rules {
                for range in &rule.1 {
                    if var <= range.1 && var >= range.0 {
                        passed = true;
                        break 'rules;
                    }
                }
            }

            if passed {
                for rule in &rules {
                    let mut failed = true;
                    for range in &rule.1 {
                        if var <= range.1 && var >= range.0 {
                            failed = false;
                        }
                    }
                    if failed {
                        positions.get_mut(&rule.0).unwrap().remove(&i);
                    }
                }
            }
        }
    }

    let mut fixed: HashMap<String, usize> = HashMap::new();

    let mut changes = true;
    while changes {
        changes = false;

        for (label, indexes) in &positions {
            if indexes.len() == 1 {
                fixed.insert(label.clone(), *indexes.iter().next().unwrap());
                changes = true;
            } 
        }

        for (label, index) in &fixed {
            positions.remove(label);

            for (_, indexes) in positions.iter_mut() {
                indexes.remove(&index);
            }
        }
    }

    let mut result = 1;
    for (label, index) in fixed {
        if label.contains("departure") {
            result *= myticket[index];
        }

    }
    
    Ok(result.to_string())
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

    #[test]
    fn test1() {
        let input = "class;: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        // dbg!(solve1(input).unwrap());
    }

    #[test]
    fn test2() {
        let input = "class;: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        dbg!(solve2(input).unwrap());
    }
}
