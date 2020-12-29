use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(
        r"(?x)
            ^
            (?P<name>\D+)
            :\s
            (?P<start0>\d+)
            -
            (?P<end0>\d+)
            \sor\s
            (?P<start1>\d+)
            -
            (?P<end1>\d+)
            $
        ",
    )
    .unwrap();
}

// const TICKET_SIZE: usize = 3; // example.txt

// const TICKET_SIZE: usize = 3; // example.txt

type Ticket = Vec<u32>;

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(u32, u32)>,
}

impl Rule {
    fn new(name: &str, ranges: Vec<(u32, u32)>) -> Self {
        Self {
            name: name.to_string(),
            ranges,
        }
    }

    fn is_num_valid(&self, ticket_num: u32) -> bool {
        for r in self.ranges.iter() {
            if ticket_num >= r.0 && ticket_num <= r.1 {
                return true;
            }
        }
        false
    }
}

fn get_ticket_error_rate(rules: &Vec<Rule>, ticket: &Ticket) -> u32 {
    let mut all_status = vec![false; ticket.len()];
    for (i, ticket_num) in ticket.iter().enumerate() {
        for rule in rules.iter() {
            if rule.is_num_valid(*ticket_num) {
                all_status[i] = true;
                break;
            }
        }
    }
    let mut error_rate = 0;
    for (status, ticket_num) in all_status.iter().zip(ticket.iter()) {
        if !status {
            error_rate += ticket_num;
        }
    }
    error_rate
}

fn parse_ticket(line: &str) -> Ticket {
    line.split(",").map(|n| n.parse::<u32>().unwrap()).collect()
}

fn read_lines(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn parse_rule(line: &str) -> Rule {
    let caps = RULE_RE.captures(&line);
    if let Some(caps) = caps {
        Rule::new(
            &caps["name"],
            vec![
                (
                    caps["start0"].parse::<u32>().unwrap(),
                    caps["end0"].parse::<u32>().unwrap(),
                ),
                (
                    caps["start1"].parse::<u32>().unwrap(),
                    caps["end1"].parse::<u32>().unwrap(),
                ),
            ],
        )
    } else {
        panic!("Failed to parse rule: {}", line);
    }
}

fn parse_nearby_tickets(lines: &Vec<String>) -> Vec<Ticket> {
    lines
        .iter()
        .skip_while(|line| *line != "nearby tickets:")
        .skip(1)
        .map(|line| parse_ticket(line))
        .collect::<Vec<Ticket>>()
}

fn parse_your_ticket(lines: &Vec<String>) -> Ticket {
    lines
        .iter()
        .skip_while(|line| *line != "your ticket:")
        .skip(1)
        .take(1)
        .map(|line| parse_ticket(line))
        .collect::<Vec<Ticket>>()[0]
        .clone()
}

fn parse_rules(lines: &Vec<String>) -> Vec<Rule> {
    lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| parse_rule(line))
        .collect::<Vec<Rule>>()
}

fn get_valid_rules(tickets: &Vec<Ticket>, rules: &Vec<Rule>, num_index: usize) -> Vec<String> {
    let mut valid_rules = Vec::new();
    for rule in rules.iter() {
        let mut rule_is_valid = true;
        for ticket in tickets.iter() {
            let ticket_num = ticket[num_index];
            if !rule.is_num_valid(ticket_num) {
                rule_is_valid = false;
                break;
            }
        }
        if rule_is_valid {
            valid_rules.push(rule.name.clone());
        }
    }
    valid_rules
}

fn assign_rules(valid_rules_per_field: &Vec<Vec<String>>) -> Vec<String> {
    // TODO: Something is broken, more than one valid rule are found...

    let mut removed_rules = HashSet::new();
    let mut assigned_rules = HashMap::new();
    println!("{:?}", valid_rules_per_field);
    loop {
        // Find field with least valid rules
        let mut best_idx = 0;
        for (idx, rules) in valid_rules_per_field.iter().enumerate() {
            let best = valid_rules_per_field[best_idx].iter().fold(0, |acc, rule| {
                acc + if removed_rules.contains(rule) { 0 } else { 1 }
            });
            let this = rules.iter().fold(0, |acc, rule| {
                acc + if removed_rules.contains(rule) { 0 } else { 1 }
            });
            if (this < best || best == 0) && this != 0 {
                best_idx = idx;
            }
        }
        let valid_rules = &valid_rules_per_field[best_idx];

        let best = valid_rules_per_field[best_idx].iter().fold(0, |acc, rule| {
            acc + if removed_rules.contains(rule) { 0 } else { 1 }
        });
        println!("best {}", best);

        // Find rules that hasn't been removed yet
        let mut potential_valid = Vec::new();
        for valid_rule in valid_rules.iter() {
            if !removed_rules.contains(valid_rule) {
                potential_valid.push(valid_rule);
            }
        }

        // Choose the rule with least occurances in other fields
        let valid_rule = potential_valid
            .iter()
            .fold(("", std::i32::MAX), |acc, rule| {
                let occurances = valid_rules_per_field.iter().fold(0, |acc, rules| {
                    acc + if rules.contains(rule) { 1 } else { 0 }
                });
                if occurances < acc.1 {
                    (rule, occurances)
                } else {
                    acc
                }
            });

        if valid_rule.1 == std::i32::MAX {
            break;
        }

        removed_rules.insert(valid_rule.0.to_string());
        assigned_rules.insert(best_idx, valid_rule.0.to_string());
    }
    let mut rules = vec!["<N/A>".to_string(); valid_rules_per_field.len()];
    for (key, rule) in assigned_rules.iter() {
        rules[*key] = rule.clone();
    }
    rules
}

fn part1(file_name: &str) {
    let lines = read_lines(file_name);
    let rules = parse_rules(&lines);
    let nearby_tickets = parse_nearby_tickets(&lines);
    let mut error_rate = 0;
    for ticket in nearby_tickets.iter() {
        error_rate += get_ticket_error_rate(&rules, ticket);
    }
    println!("Part1: {}", error_rate);
}

fn part2(file_name: &str) {
    let lines = read_lines(file_name);
    let rules = parse_rules(&lines);
    let nearby_tickets = parse_nearby_tickets(&lines);
    let nearby_no_error = nearby_tickets
        .iter()
        .filter(|t| get_ticket_error_rate(&rules, t) == 0)
        .map(|t| t.clone())
        .collect();
    let your_ticket = parse_your_ticket(&lines);
    let num_ticket_fields = your_ticket.len();

    let mut valid_rules = Vec::new();
    for i in 0..num_ticket_fields {
        let valid_for_field = get_valid_rules(&nearby_no_error, &rules, i);
        valid_rules.push(valid_for_field);
    }
    let assigned_rules = assign_rules(&valid_rules);

    println!("Your ticket:");
    let mut answer = 1;
    for (name, number) in assigned_rules.iter().zip(your_ticket.iter()) {
        println!("{}: {}", name, number);
        if name.starts_with("departure") {
            answer *= (*number) as u64;
        }
    }
    println!("Part2: {}", answer);
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
