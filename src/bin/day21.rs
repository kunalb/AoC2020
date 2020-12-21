use std::io::{self, Read};
use std::error::Error;
use std::env;

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

struct Details<'a> {
    constraints: Vec<(Vec<&'a str>, Vec<&'a str>)>,
    possibilities: HashMap<&'a str, HashSet<&'a str>>,
    all_foods: HashSet<&'a str>,
    unused_foods: HashSet<&'a str>,
}

fn extract_details(buffer: &str) -> Result<Details, Box<dyn Error>> {
    let mut constraints: Vec<(Vec<&str>, Vec<&str>)> = Vec::new();
    let mut possibilities: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut all_foods: HashSet<&str> = HashSet::new();

    for line in buffer.lines() {
        let pieces = line.split("(contains").collect::<Vec<_>>();
        let foods = pieces[0].trim().split(" ").collect::<HashSet<_>>();
        let allergens = pieces[1][..pieces[1].len() - 1].trim().split(", ").collect::<Vec<_>>();

        all_foods.extend(foods.clone());
        constraints.push((foods.clone().into_iter().collect(), allergens.clone()));

        for allergen in allergens {
            let entry = possibilities.entry(allergen).or_insert(foods.clone());
            *entry = entry
                .intersection(&foods)
                .map(|x| *x)
                .collect::<HashSet<_>>();
        }
    }

    let used_foods = possibilities
        .values()
        .fold(HashSet::new(),
              |a, x| a.union(x).map(|x| *x).collect());

    let unused_foods =
        all_foods.difference(&used_foods).copied().collect::<HashSet<_>>();

    Ok(Details {
        constraints, possibilities, all_foods, unused_foods
    })
}

fn solve1(buffer: &str) -> Result<usize, Box<dyn Error>> {
    let details = extract_details(buffer)?;
    Ok(details.constraints
        .iter()
        .map(|(f, a)| f.iter().filter(|x| details.unused_foods.contains(**x)).count())
        .sum())
}

fn solve2(buffer: &str) -> Result<String, Box<dyn Error>> {
    let mut details = extract_details(buffer)?;

    for mut possibility in details.possibilities.iter_mut() {
        let next = possibility.1.difference(&details.unused_foods)
            .map(|x| *x)
            .collect::<HashSet<_>>();
        possibility.1.clear();
        possibility.1.extend(next);
    }

    let mut assignments: HashMap<&str, &str> = HashMap::new();
    loop {
        let mut new_assignments = HashMap::new();
        for p in &details.possibilities {
            if p.1.len() == 1 {
                new_assignments.insert(*p.0, *(p.1.iter().next().unwrap()));
            }
        }

        for n in new_assignments.iter() {
            details.possibilities.remove(*n.0);
        }

        for n in &new_assignments {
            for p in details.possibilities.iter_mut() {
                p.1.remove(*n.1);
            }
        }

        assignments.extend(new_assignments);

        if details.possibilities.len() == 0 {
            break;
        }
    }

    Ok(
        assignments.iter().sorted_by_key(|x| x.0)
            .map(|x| *x.1)
            .collect::<Vec<&str>>()
            .join(",")
    )
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