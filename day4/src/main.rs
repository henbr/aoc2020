use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;

lazy_static! {
    static ref HGT_RE: Regex = Regex::new(
        r"(?x)
            (?P<height>\d+)  # the year
            (?P<unit>\D+) # the month
        ",
    )
    .unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^#[[:xdigit:]]{6}$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
}

struct Passport {
    pairs: Vec<(String, String)>,
}

impl Passport {
    fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    fn add_field(&mut self, key: &str, value: &str) {
        self.pairs.push((key.to_string(), value.to_string()));
    }

    fn has_field(&self, field: &str) -> bool {
        let pair = self.pairs.iter().position(|p| p.0 == field);
        pair.is_some()
    }

    fn has_all_fields(&self) -> bool {
        self.has_field("byr")
            && self.has_field("iyr")
            && self.has_field("eyr")
            && self.has_field("hgt")
            && self.has_field("hcl")
            && self.has_field("ecl")
            && self.has_field("pid")
    }

    fn get_value(&self, field: &str) -> Option<String> {
        let pair = self.pairs.iter().position(|p| p.0 == field);
        if let Some(pair) = pair {
            Some(self.pairs[pair].1.clone())
        } else {
            None
        }
    }

    fn is_all_fields_valid(&self) -> bool {
        let byr = self.get_value("byr").unwrap();
        let byr_num = byr.parse::<u32>().unwrap_or(0);
        let byr_ok = byr.len() == 4 && byr_num >= 1920 && byr_num <= 2002;

        let iyr = self.get_value("iyr").unwrap();
        let iyr_num = iyr.parse::<u32>().unwrap_or(0);
        let iyr_ok = iyr.len() == 4 && iyr_num >= 2010 && iyr_num <= 2020;

        let eyr = self.get_value("eyr").unwrap();
        let eyr_num = eyr.parse::<u32>().unwrap_or(0u32);
        let eyr_ok = eyr.len() == 4 && eyr_num >= 2020 && eyr_num <= 2030;

        let hgt = self.get_value("hgt").unwrap();
        let caps = HGT_RE.captures(&hgt);
        let hgt_ok = if let Some(caps) = caps {
            let height = caps["height"].parse::<u32>().unwrap_or(0u32);
            let unit = &caps["unit"];
            (unit == "cm" && height >= 150 && height <= 193)
                || (unit == "in" && height >= 59 && height <= 76)
        } else {
            false
        };

        let hcl = self.get_value("hcl").unwrap();
        let hcl_ok = HCL_RE.is_match(&hcl);

        let ecl = self.get_value("ecl").unwrap();
        let ecl_ok = ecl == "amb"
            || ecl == "blu"
            || ecl == "brn"
            || ecl == "gry"
            || ecl == "grn"
            || ecl == "hzl"
            || ecl == "oth";

        let pid = self.get_value("pid").unwrap();
        let pid_ok = PID_RE.is_match(&pid);

        /*
        println!(
            "byr:{}({}), iyr:{}({}), eyr:{}({}), hgt:{}({}), hcl:{}({}), ecl:{}({}), pid:{}({})",
            byr_ok,
            byr,
            iyr_ok,
            iyr,
            eyr_ok,
            eyr,
            hgt_ok,
            hgt,
            hcl_ok,
            hcl,
            ecl_ok,
            ecl,
            pid_ok,
            pid,
        );
        */

        return byr_ok && iyr_ok && eyr_ok && hgt_ok && hcl_ok && ecl_ok && pid_ok;
    }
}

fn read(file: &str) -> Vec<Passport> {
    let data = fs::read_to_string(file).unwrap();
    let mut passports = Vec::new();
    let mut passport = Passport::new();
    for line in data.lines() {
        if line.is_empty() {
            passports.push(passport);
            passport = Passport::new();
        } else {
            for pair in line.split(' ') {
                let key_value = pair.split(':').collect::<Vec<&str>>();
                let key = key_value.get(0).unwrap();
                let value = key_value.get(1).unwrap();
                passport.add_field(key, value);
            }
        }
    }
    println!("Num passports: {}", passports.len());
    passports
}

fn part1(file: &str) {
    let passports = read(file);
    let num_valid = passports.iter().fold(0, |sofar, passport| {
        if passport.has_all_fields() {
            sofar + 1
        } else {
            sofar
        }
    });
    println!("Part1: {}", num_valid);
}

fn part2(file: &str) {
    let passports = read(file);
    let num_valid = passports.iter().fold(0, |sofar, passport| {
        if passport.has_all_fields() && passport.is_all_fields_valid() {
            sofar + 1
        } else {
            sofar
        }
    });
    println!("Part2: {}", num_valid);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Reading file: {}", args[1]);
        part1(&args[1]);
        part2(&args[1]);
    } else {
        println!("No input file specified");
    }
}
