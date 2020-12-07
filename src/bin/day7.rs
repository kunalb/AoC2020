use std::io::{self, Read};
use std::error::Error;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Bags<'a> = HashMap<&'a str, HashMap<&'a str, usize>>;

fn parse(buffer: &str) -> Result<(Bags, Bags), Box<dyn Error>> {
    let mut contained: Bags = HashMap::new();
    let mut containers: Bags = HashMap::new();

    for line in buffer.lines() {
        let pieces = line.split(" contain ").collect::<Vec<_>>();

        let name = &pieces[0][..pieces[0].len()-1];
        let container = containers.entry(name).or_insert(HashMap::new());

        if pieces[1] != "no other bags." {
            for piece in pieces[1][..pieces[1].len()-1].split(", ") {
                let count = &piece[0..1].parse::<usize>()?;
                let n = &piece[2..].trim_end_matches('s');
                container.entry(n).or_insert(*count);
                contained.entry(n).or_insert(HashMap::new()).entry(name).or_insert(*count);
            }
        }
    }

    Ok((contained, containers))
}

fn solve1(buffer: &str) -> Result<usize, Box<dyn Error>> {
    let (contained, _) = parse(buffer)?;

    let mut visited = HashSet::new();
    let mut q = VecDeque::new();

    q.push_back("shiny gold bag");
    while !q.is_empty() {
        let top = q.pop_front().unwrap();
        if visited.contains(top) {
            continue;
        }

        if let Some(contained_by) = contained.get(top) { 
            q.extend(contained_by.keys());
        }

        visited.insert(top);
    }

    Ok(visited.len() - 1)
}

fn solve2(buffer: &str) -> Result<usize, Box<dyn Error>> {
    let (_, containers) = parse(buffer)?;

    let mut q = VecDeque::new();
    q.push_back(("shiny gold bag", 1));

    let mut count: usize = 0;
    while !q.is_empty() {
        let top = q.pop_front().unwrap();
        count += top.1;
        if let Some(inner_bags) = containers.get(top.0) {
            for (key, val) in inner_bags {
                q.push_back((key, val * top.1));
            }
        }
    } 
        
    Ok(count - 1)
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
