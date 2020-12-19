use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::{self, Read};

fn parse(buffer: &str) -> (HashMap<&str, Vec<Vec<&str>>>, Vec<&str>) {
    let pieces = buffer.split("\n\n").collect::<Vec<_>>();

    let rules = pieces[0]
        .split("\n")
        .map(|x| x.split(":").collect::<Vec<_>>())
        .map(|v| {
            (
                v[0],
                v[1].split("|")
                    .map(|p| p.split(" ").filter(|x| x != &"").collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    let messages = pieces[1].split("\n").collect::<Vec<_>>();

    (rules, messages)
}

fn apply<'a>(
    r: &str,
    rules: &'a HashMap<&'a str, Vec<Vec<&'a str>>>,
    msg: &'a str,
    prefix: String,
) -> Vec<&'a str> {
    let rule = rules.get(r).unwrap();
    let mut results: Vec<&'a str> = vec![];
    //println!("{}{} {:?} {}", prefix, r, rule, msg);

    for part in rule {
        let mut leftovers = vec![msg];
        let mut next_leftovers = vec![];

        for item in part {
            for leftover in &leftovers {
                if let Ok(x) = item.parse::<u64>() {
                    let x = x.to_string();
                    let inner_results = apply(&x, rules, leftover, format!("{} ", prefix));

                    for inner_result in inner_results {
                        if &inner_result != leftover {
                            next_leftovers.push(inner_result);
                        }
                    }
                } else {
                    if leftover.starts_with(&item[1..item.len() - 1]) {
                        next_leftovers.push(&leftover[1..]);
                    }
                }
            }

            leftovers = next_leftovers;
            next_leftovers = vec![];
        }

        results.append(&mut leftovers);
    }

    //println!("{}{} {} > {:?}", prefix, r, msg, results);
    results
}

fn solve1(buffer: &str) -> usize {
    let (rules, messages) = parse(buffer);
    messages
        .iter()
        .filter(|m| {
            apply("0", &rules, m, String::from(""))
                .iter()
                .any(|x| x == &"")
        })
        .count()
}

fn solve2(buffer: &str) -> usize {
    let (mut rules, messages) = parse(buffer);
    rules.insert("8", vec![vec!["42"], vec!["42", "8"]]);
    rules.insert("11", vec![vec!["42", "31"], vec!["42", "11", "31"]]);

    messages
        .iter()
        .filter(|m| {
            apply("0", &rules, m, String::from(""))
                .iter()
                .any(|x| x == &"")
        })
        .count()
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

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    const INPUT2: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT2), 12);
    }

    #[test]
    fn test3() {
        assert_eq!(solve2(
            r#"0: 1 | 1 0
1: "a"

aa
"#
        ), 1);
    }

    #[test]
    fn test4() {
        assert_eq!(solve2(
            r#"0: 3 2
1: "a"
2: "b"
3: 1 1 | 1 3 2

aaabb
"#
        ), 1);
    }
}
