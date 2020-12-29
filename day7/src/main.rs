use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(
        r#"(?x)
            ^
            (?P<color>.+)
            \sbags\scontain\s
            (?P<content>.+)
            \.$
        "#,
    )
    .unwrap();
    static ref CONTENT_RE: Regex = Regex::new(
        r#"(?x)
            ^
            (?P<num>\d+)
            \s
            (?P<color>.+)
            \sbag
        "#,
    )
    .unwrap();
}

fn read(file: &str) -> Vec<String> {
    let data = fs::read_to_string(file).unwrap();
    data.lines().map(|l| l.to_string()).collect()
}

struct Bag {
    color: String,
    bags: Vec<(i32, String)>,
}

impl Bag {
    fn new(color: &str) -> Self {
        Self {
            color: color.to_string(),
            bags: Vec::new(),
        }
    }

    fn add_bag(&mut self, num: i32, color: &str) {
        self.bags.push((num, color.to_string()))
    }

    /*
    fn print(&self) {
        let other = self
            .bags
            .iter()
            .map(|b| format!("{}x'{}'", b.0, b.1))
            .collect::<Vec<String>>()
            .join(", ");
        println!("'{}': {}", self.color, other);
    }
    */
}

fn read_bags(file: &str) -> Vec<Bag> {
    let lines = read(file);
    let mut bags = Vec::new();
    for l in lines.iter() {
        let caps = LINE_RE.captures(l);
        if let Some(caps) = caps {
            let color = &caps["color"];
            let content = &caps["content"];
            let mut bag = Bag::new(color);
            if content != "no other bags" {
                for other in content.split(", ") {
                    let caps = CONTENT_RE.captures(other);
                    if let Some(caps) = caps {
                        let num = caps["num"].parse::<i32>().unwrap();
                        let color = &caps["color"];
                        bag.add_bag(num, color);
                    } else {
                        println!("No match: '{}'", other);
                    }
                }
            }
            bags.push(bag);
        } else {
            println!("No match: {}", l);
        }
    }
    //for bag in bags.iter() {
    //    bag.print();
    //}
    bags
}

fn part1(file: &str) {
    let bags = read_bags(file);
    let mut count = 0;
    for bag in bags.iter() {
        if bag.color == "shiny gold" {
            continue;
        }
        let mut bags_to_visit = vec![bag.color.clone()];
        while bags_to_visit.len() > 0 {
            let other_color = bags_to_visit.pop().unwrap();
            let idx = bags.iter().position(|b| b.color == other_color);
            if let Some(idx) = idx {
                let other_bag = &bags[idx];
                bags_to_visit.extend(other_bag.bags.iter().map(|b| b.1.clone()));
            }
            if other_color == "shiny gold" {
                count += 1;
                break;
            }
        }
    }

    println!("Part 1: {} found", count);
}

fn part2(file: &str) {
    let bags = read_bags(file);
    let mut count = 0;
    let idx = bags.iter().position(|b| b.color == "shiny gold").unwrap();
    let bag = &bags[idx];
    let mut bags_to_visit = bag
        .bags
        .iter()
        .map(|b| (b.0, b.1.clone()))
        .collect::<Vec<(i32, String)>>();
    while bags_to_visit.len() > 0 {
        let (num, other_color) = bags_to_visit.pop().unwrap();
        count += num;
        let idx = bags.iter().position(|b| b.color == other_color);
        if let Some(idx) = idx {
            let other_bag = &bags[idx];
            if !other_bag.bags.is_empty() {
                bags_to_visit.extend(other_bag.bags.iter().map(|b| (b.0 * num, b.1.clone())));
            }
        }
    }
    println!("Part 2: {} found", count);
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
