use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::io::{self, Read};

fn read_deck(piece: &str) -> VecDeque<u64> {
    piece
        .trim()
        .lines()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<VecDeque<_>>()
}

fn score(deck: &VecDeque<u64>) -> u64 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i + 1) as u64 * x)
        .sum()
}

fn solve1(buffer: &str) -> Result<u64, Box<dyn Error>> {
    let mut decks = buffer.split("\n\n").map(read_deck).collect::<Vec<_>>();

    while decks.iter().all(|x| x.len() > 0) {
        let p1 = decks[0].pop_front().unwrap();
        let p2 = decks[1].pop_front().unwrap();

        if p1 > p2 {
            decks[0].push_back(p1);
            decks[0].push_back(p2);
        } else if p2 > p1 {
            decks[1].push_back(p2);
            decks[1].push_back(p1);
        } else {
            unreachable!();
        }
    }

    let winner = if decks[0].len() > 0 {
        &decks[0]
    } else {
        &decks[1]
    };

    Ok(score(winner))
}

fn recursive_combat(deck1: VecDeque<u64>, deck2: VecDeque<u64>) -> (usize, VecDeque<u64>) {
    let mut rounds = HashSet::new();
    let mut deck1 = deck1;
    let mut deck2 = deck2;

    while deck1.len() > 0 && deck2.len() > 0 {
        let p1 = deck1.pop_front().unwrap();
        let p2 = deck2.pop_front().unwrap();

        let key = deck1
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
            + "&"
            + &deck2
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",");
        if rounds.contains(&key) {
            return (1, deck1);
        } else {
            rounds.insert(key);
        }

        let winner;
        if p1 <= deck1.len() as u64 && p2 <= deck2.len() as u64 {
            let results = recursive_combat(
                (0..(p1 as usize))
                    .map(|x| deck1[x])
                    .collect::<VecDeque<_>>(),
                (0..(p2 as usize))
                    .map(|x| deck2[x])
                    .collect::<VecDeque<_>>(),
            );
            winner = results.0;
        } else {
            if p1 > p2 {
                winner = 1;
            } else if p2 > p1 {
                winner = 2;
            } else {
                unreachable!();
            }
        }

        if winner == 1 {
            deck1.push_back(p1);
            deck1.push_back(p2);
        } else if winner == 2 {
            deck2.push_back(p2);
            deck2.push_back(p1);
        } else {
            unreachable!();
        }
    }

    if deck1.len() > 0 {
        (1, deck1)
    } else {
        (2, deck2)
    }
}

fn solve2(buffer: &str) -> Result<u64, Box<dyn Error>> {
    let decks = buffer.split("\n\n").map(read_deck).collect::<Vec<_>>();
    let (_winner, deck) = recursive_combat(decks[0].clone(), decks[1].clone());
    Ok(score(&deck))
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

    const INPUT: &'static str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT).unwrap(), 291);
    }
}
