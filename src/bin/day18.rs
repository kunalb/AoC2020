use std::env;
use std::error::Error;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Val {
    Num(i64),
    Add,
    Mul,
    Open,
    Close,
}

fn solve1(buffer: &str) -> Result<i64, Box<dyn Error>> {
    let mut result: i64 = 0;
    for line in buffer.lines() {
        let tokens = line.trim().chars();
        let mut stack: Vec<Val> = vec![];

        for token in tokens {
            match token {
                '+' => stack.push(Val::Add),
                '*' => stack.push(Val::Mul),
                '(' => stack.push(Val::Open),
                ')' => stack.push(Val::Close),
                x if x.is_digit(10) => {
                    let x = x.to_digit(10).unwrap() as i64;
                    if let Some(Val::Num(y)) = stack.last().copied() {
                        stack.pop();
                        stack.push(Val::Num(y * 10 + x));
                    } else {
                        stack.push(Val::Num(x));
                    };
                }
                ' ' => {}
                _ => panic!("Unrecognized {}", token),
            }

            while stack.len() >= 3 {
                match stack[stack.len() - 3..] {
                    [Val::Open, a, Val::Close] => {
                        stack.truncate(stack.len() - 3);
                        stack.push(a);
                    }
                    [Val::Num(a), Val::Add, Val::Num(b)] => {
                        stack.truncate(stack.len() - 3);
                        stack.push(Val::Num(a + b))
                    }
                    [Val::Num(a), Val::Mul, Val::Num(b)] => {
                        stack.truncate(stack.len() - 3);
                        stack.push(Val::Num(a * b))
                    }
                    _ => {
                        break;
                    }
                };
            }
        }

        assert!(stack.len() == 1, "{:?}", stack);
        if let Some(Val::Num(x)) = stack.last() {
            result += x;
        }
    }

    Ok(result)
}

#[derive(Debug)]
enum PTree {
    Term(i64),
    Add(Vec<PTree>),
    Mul(Vec<PTree>),
}

impl PTree {
    fn eval(&self) -> i64 {
        match self {
            PTree::Term(x) => *x,
            PTree::Add(v) => v.iter().map(|a| a.eval()).sum(),
            PTree::Mul(v) => v.iter().map(|a| a.eval()).product(),
        }
    }
}

fn solve2(buffer: &str) -> Result<i64, Box<dyn Error>> {
    let mut result: i64 = 0;
    for line in buffer.lines() {
        let tokens = line.trim().chars();
        let mut stack: Vec<Val> = vec![];

        for token in tokens {
            match token {
                '+' => stack.push(Val::Add),
                '*' => stack.push(Val::Mul),
                '(' => stack.push(Val::Open),
                ')' => stack.push(Val::Close),
                x if x.is_digit(10) => {
                    let x = x.to_digit(10).unwrap() as i64;
                    if let Some(Val::Num(y)) = stack.last().copied() {
                        stack.pop();
                        stack.push(Val::Num(y * 10 + x));
                    } else {
                        stack.push(Val::Num(x));
                    };
                }
                ' ' => {}
                _ => panic!("Unrecognized {}", token),
            }
        }

        let mut tree_stack = vec![PTree::Add(vec![PTree::Term(0)])];
        let mut brace_stack: Vec<usize> = vec![];

        // dbg!(&stack);
        for token in stack {
            // dbg!(&tree_stack);
            // dbg!(token);

            match (tree_stack.last_mut().unwrap(), token) {
                (PTree::Add(v), Val::Open) | (PTree::Mul(v), Val::Open) if v.len() == 1 => {
                    let next_tree = PTree::Add(vec![PTree::Term(0)]);
                    tree_stack.push(next_tree);
                    brace_stack.push(tree_stack.len() - 1);
                }
                (_, Val::Close) => {
                    let pop_to = brace_stack.pop().unwrap();
                    let mut r: Option<i64> = None;
                    while tree_stack.len() > pop_to {
                        let mut tree = tree_stack.pop().unwrap();
                        match &mut tree {
                            PTree::Add(v) | PTree::Mul(v) if v.len() == 1 => {
                                v.push(PTree::Term(r.unwrap()))
                            }
                            _ => {}
                        }
                        r = Some(tree.eval());
                    }

                    let cur = PTree::Term(r.unwrap());
                    match tree_stack.last_mut().unwrap() {
                        PTree::Add(v) if v.len() == 1 => v.push(cur),
                        PTree::Mul(v) if v.len() == 1 => v.push(cur),
                        _ => panic!("{:?} {:?}", tree_stack, token),
                    }
                }
                (PTree::Add(v), Val::Num(x)) if v.len() == 1 => {
                    v.push(PTree::Term(x));
                }
                (PTree::Mul(v), Val::Num(x)) if v.len() == 1 => {
                    v.push(PTree::Term(x));
                }
                (PTree::Add(v), Val::Add) if v.len() == 2 => {
                    let cur = tree_stack.pop().unwrap();
                    tree_stack.push(PTree::Add(vec![cur]));
                }
                (PTree::Mul(v), Val::Mul) if v.len() == 2 => {
                    let cur = tree_stack.pop().unwrap();
                    tree_stack.push(PTree::Mul(vec![cur]));
                }
                (PTree::Add(v), Val::Mul) if v.len() == 2 => {
                    let cur = tree_stack.pop().unwrap();
                    tree_stack.push(PTree::Mul(vec![cur]));
                }
                (PTree::Mul(v), Val::Add) if v.len() == 2 => {
                    let next_tree = PTree::Add(vec![v.pop().unwrap()]);
                    tree_stack.push(next_tree);
                }
                _ => panic!("{:?} {:?}", tree_stack, token),
            }
        }
        // dbg!(&tree_stack);

        let mut r: Option<i64> = None;
        for mut tree in tree_stack.into_iter().rev() {
            match &mut tree {
                PTree::Add(v) | PTree::Mul(v) if v.len() == 1 => v.push(PTree::Term(r.unwrap())),
                _ => {}
            }

            r = Some(tree.eval());
        }

        result += r.unwrap();
    }

    Ok(result)
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
        assert_eq!(solve1("1 + 2 * 3 + 4 * 5 + 6").unwrap(), "71");
    }

    #[test]
    fn test2() {
        assert_eq!(solve2("1 + 2 * 3 + 4 * 5 + 6").unwrap(), "231");
    }

    #[test]
    fn test_tree() {
        assert_eq!(solve2("2 * 3 + 5").unwrap(), "16");
    }

    #[test]
    fn test_par() {
        assert_eq!(solve2("2 * 3 + (4 * 5)").unwrap(), "46");
    }

    #[test]
    fn test_nested_sum() {
        assert_eq!(solve2("((1 * 2) + (3 * 4) + 5) + 6").unwrap(), "25");
    }

    #[test]
    fn test_complex() {
        assert_eq!(solve2("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap(), "1445");
        assert_eq!(
            solve2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap(),
            "669060"
        );
        assert_eq!(
            solve2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap(),
            "23340"
        );
        assert_eq!(solve2("(5 * 2 + 3) + 4").unwrap(), "29");
    }
}
