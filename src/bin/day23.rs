use std::env;
use std::error::Error;
use std::io::{self, Read};
use std::collections::HashSet;

fn solve1(buffer: &str) -> Result<String, Box<dyn Error>> {
    let mut labels = buffer
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let len = labels.len();

    let mut next_labels = vec![100; len];
    let mut current_cup = 0;

    // Clockwise = +ve in array
    for _ in 0..100 {
        let picked = (1..=3)
            .map(|x| (current_cup + x) % len)
            .map(|x| labels[x])
            .collect::<HashSet<_>>();

        let mut destination = labels[current_cup] - 1;
        if destination == 0 {
            destination += len as u32;
        }

        while picked.contains(&destination) {
            destination -= 1;
            if destination == 0 {
                destination = len as u32;
            }
        }

        next_labels[current_cup] = labels[current_cup];
        let mut label_offset = (current_cup + 4) % len;
        let mut next_offset = (current_cup + 1) % len;

        loop {
            let should_break = labels[label_offset] == destination;

            next_labels[next_offset] = labels[label_offset];
            label_offset = (label_offset + 1) % len;
            next_offset = (next_offset + 1) % len;

            if should_break {
                break;
            }
        }

        for i in 1..=3  {
            next_labels[next_offset] = labels[(current_cup + i) % len];
            next_offset = (next_offset + 1) % len;
        }

        while label_offset != current_cup {
            next_labels[next_offset] = labels[label_offset];
            label_offset = (label_offset + 1) % len;
            next_offset = (next_offset + 1) % len;
        }
        
        current_cup += 1;
        if current_cup == len {
            current_cup = 0;
        }

        let temp = labels;
        labels = next_labels;
        next_labels = temp;
    }

    let start_pos = labels.iter().position(|x| *x == 1).unwrap();
    let mut result = String::from("");
    for offset in 1..=8 {
        result += &labels[(start_pos + offset) % len].to_string();
    }

    Ok(result)
}

const ITER: usize = 10_000_000;
const LEN: usize = 1_000_000;

#[inline]
fn label_modulo(x: u32) -> u32 {
    if x == 0 {
        LEN as u32
    } else {
        x
    }
}

fn solve2(buffer: &str) -> Result<u64, Box<dyn Error>> {
    let input = buffer
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut labels = [0; LEN];
    for (i, &l) in input.iter().enumerate() {
        labels[i] = l;
    }
    for i in 9..LEN {
        labels[i] = (i + 1) as u32;
    }

    let mut picked: [u32; 4] = [0; 4];

    let mut val = 0;
    let mut start_pos = labels.iter().position(|&x| x == 1).unwrap();

    // Current cup is always at position 0
    // Clockwise = +ve in array
    for p in 0..ITER {
        // println!("{:?}", labels);

        picked.copy_from_slice(&labels[0..4]);
        let mut dest_label = label_modulo(labels[0] - 1);
        while picked.iter().position(|&x| x == dest_label).is_some() {
            dest_label = label_modulo(dest_label - 1);
        }
        // println!("{}", dest_label);

        let dest_pos = labels.iter().position(|&x| x == dest_label).unwrap();
        labels.copy_within(4..=dest_pos, 0);

        let new_dest_pos = dest_pos - 4;
        labels[new_dest_pos + 1..=new_dest_pos + 3].copy_from_slice(&picked[1..]);
        labels.copy_within(dest_pos + 1..LEN, dest_pos);
        labels[LEN - 1] = picked[0];


        if let Some(p_pos) = picked.iter().position(|&x| x == 1) {
            if p_pos == 0 {
                start_pos = LEN - 1;
            } else {
                start_pos = p_pos + new_dest_pos;
            }
        } else if start_pos <= dest_pos {
            start_pos = (start_pos - 4) % LEN;
        } else {
            start_pos -= 1;
        }
        assert_eq!(labels[start_pos], 1);

        let a = labels[(start_pos + 1) % LEN] as u64;
        let b = labels[(start_pos + 2) % LEN] as u64;
        let new_val = a * b;
        if val != new_val {
            println!("{:4} | {:10} | {:10} * {:10} = {:10} | ", p + 1, start_pos, a, b, new_val);
            val = new_val;
        }
    }

    let start_pos = labels.iter().position(|x| *x == 1).unwrap();
    let a = labels[(start_pos + 1) % LEN] as u64;
    let b = labels[(start_pos + 2) % LEN] as u64;
    Ok(a * b)
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
        dbg!(solve1("389125467"));
    }

    #[test]
    fn test2() {
        dbg!(solve2("389125467"));
    }
}
