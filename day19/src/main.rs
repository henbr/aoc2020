use std::env;
use std::fs;

#[derive(Clone, Copy, Debug)]
enum Part {
    Terminal(char),
    Reference(usize),
}

type Rule = Vec<Vec<Part>>;

fn read_file(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn parse_rules(lines: &Vec<String>) -> Vec<Rule> {
    let mut rules = vec![Rule::new(); lines.len()];
    for line in lines.iter() {
        if line.is_empty() {
            break;
        }
        let index_rule = line.split(": ").collect::<Vec<&str>>();
        let index = index_rule[0].parse::<usize>().unwrap();
        let rule = index_rule[1]
            .split(" | ")
            .map(|r| {
                r.split(' ')
                    .map(|t| match t.chars().nth(0).unwrap() {
                        '"' => Part::Terminal(t.chars().nth(1).unwrap()),
                        '0'..='9' => Part::Reference(t.parse::<usize>().unwrap()),
                        _ => panic!("Syntax error {}", t),
                    })
                    .collect()
            })
            .collect();
        rules[index] = rule;
    }
    rules
}

fn parse_messages(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines
        .iter()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| l.chars().collect())
        .collect()
}

fn check_with_rule(
    rules: &Vec<Rule>,
    message: &Vec<char>,
    msg_pos: usize,
    rule_idx: usize,
) -> (usize, bool) {
    let rule = &rules[rule_idx];
    if msg_pos >= message.len() {
        return (msg_pos, false);
    }
    for seq in rule.iter() {
        let mut ok = true;
        let mut pos = msg_pos;
        for p in seq.iter() {
            if pos == message.len() {
                ok = false;
                break;
            }
            match p {
                Part::Reference(idx) => {
                    let res = check_with_rule(rules, message, pos, *idx);
                    if !res.1 {
                        ok = false;
                        break;
                    }
                    pos = res.0;
                }
                Part::Terminal(c) => {
                    if *c != message[pos] {
                        ok = false;
                        break;
                    }
                    pos += 1;
                }
            }
        }
        if ok {
            return (pos, ok);
        }
    }
    (msg_pos, false)
}

fn check_message(rules: &Vec<Rule>, message: &Vec<char>, rule_idx: usize) -> bool {
    let res = check_with_rule(&rules, message, 0, rule_idx);
    res.0 == message.len() && res.1
}

fn part1(file_name: &str) {
    let lines = read_file(file_name);
    let rules = parse_rules(&lines);
    let messages = parse_messages(&lines);
    let num_ok = messages.iter().fold(0, |acc, message| {
        if check_message(&rules, message, 0) {
            acc + 1
        } else {
            acc
        }
    });
    println!("Part1: {}", num_ok);
}

// This works because the only rule that is refering to rule 8 and 11 is
// rule 0, which means this can be solved by turning rule 0 to "x*42 y*31"
// where 2 <= x and 1 <= y < x.
fn part2(file_name: &str) {
    let lines = read_file(file_name);
    let rules = parse_rules(&lines);
    let messages = parse_messages(&lines);
    let num_ok = messages.iter().fold(0, |acc, message| {
        // i is large enough to match the longest message
        for i in 2..100 {
            let mut res_42 = (0, false);
            for _ in 0..i {
                res_42 = check_with_rule(&rules, message, res_42.0, 42);
            }
            let mut res_31 = res_42;
            for _ in 0..i - 1 {
                res_31 = check_with_rule(&rules, message, res_31.0, 31);
                if res_31.0 >= message.len() {
                    break;
                }
            }
            if res_42.1 && res_31.1 && res_31.0 == message.len() {
                return acc + 1;
            }
        }
        acc
    });
    println!("Part2: {}", num_ok);
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
