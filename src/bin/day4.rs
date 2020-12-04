use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::io::{self, Read};
use std::iter;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn range_check(val: &str, min: i64, max: i64) -> bool {
    if let Ok(val) = val.parse::<i64>() {
        min <= val && val <= max
    } else {
        false
    }
}

fn verify(key: &str, val: &str) -> bool {
    lazy_static! {
        static ref HCL_RE: Regex = Regex::new(r#"^#[a-f0-9]{6}$"#).unwrap();
        static ref PID_RE: Regex = Regex::new(r#"^[0-9]{9}$"#).unwrap();
        static ref ECL_SET: HashSet<&'static str> =
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .cloned()
                .collect();
    }

    match key {
        "byr" => range_check(val, 1920, 2002),
        "iyr" => range_check(val, 2010, 2020),
        "eyr" => range_check(val, 2020, 2030),
        "hgt" => {
            let height = &val[..val.len() - 2];
            match &val[val.len() - 2..] {
                "in" => range_check(height, 59, 76),
                "cm" => range_check(height, 150, 193),
                _ => false,
            }
        }
        "hcl" => HCL_RE.is_match(val),
        "ecl" => ECL_SET.contains(val),
        "pid" => PID_RE.is_match(val),
        "cid" => true,
        _ => false,
    }
}

fn solve(buffer: &str, validate: bool) -> Result<String, Box<dyn Error>> {
    let mut valid_passports = 0;
    let req: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .iter()
        .cloned()
        .collect();

    let mut current = req.clone();
    for line in buffer.lines().chain(iter::once("")) {
        if line == "" {
            if current.is_empty() || (current.len() == 1 && current.contains("cid")) {
                valid_passports += 1;
            }
            current = req.clone();
        } else {
            for piece in line.split(" ") {
                let (key, val) = piece.split(":").collect_tuple().unwrap();
                if !validate || verify(key, val) {
                    current.remove(key);
                }
            }
        }
    }

    Ok(format!("{}", valid_passports))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "2" {
        println!("{}", solve(&buffer, true)?);
    } else {
        println!("{}", solve(&buffer, false)?);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let test_input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!("2", solve(test_input, false).unwrap());
    }

    #[test]
    fn test2() {
        let tests = "eyr;:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!("4", solve(tests, true).unwrap());
    }
}
