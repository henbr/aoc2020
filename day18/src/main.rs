use std::env;
use std::fs;

// Reference: https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
fn eval_expression(
    line: &Vec<char>,
    pos: &mut usize,
    min_bp: u32,
    precedence: fn(char) -> (u32, u32),
) -> u64 {
    let c = line[*pos];
    *pos += 1;
    let mut ans = match c {
        '(' => {
            let lhs = eval_expression(line, pos, 0, precedence);
            *pos += 1; // skip )
            lhs
        }
        '0'..='9' => c.to_digit(10).unwrap() as u64,
        _ => panic!("Unexpected token: {}", c),
    };
    loop {
        if *pos == line.len() || line[*pos] == ')' {
            break;
        }
        let (lhs_bp, rhs_bp) = precedence(line[*pos]);
        if lhs_bp < min_bp {
            break;
        }
        let op = line[*pos];
        *pos += 1;
        let rhs = eval_expression(line, pos, rhs_bp, precedence);
        ans = match op {
            '+' => ans + rhs,
            '*' => ans * rhs,
            _ => panic!("Invalid op"),
        }
    }
    ans
}

fn eval_file(file_name: &str, precedence: fn(char) -> (u32, u32)) -> u64 {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| {
            eval_expression(
                &l.chars().filter(|c| !c.is_whitespace()).collect(),
                &mut 0,
                0,
                precedence,
            )
        })
        .fold(0, |acc, e| acc + e)
}

fn part1(file_name: &str) {
    let sum = eval_file(file_name, |c| match c {
        '*' => (1, 2),
        '+' => (1, 2),
        _ => panic!("Expected op, got: {}", c),
    });
    println!("Part 1: {}", sum);
}

fn part2(file_name: &str) {
    let sum = eval_file(file_name, |c| match c {
        '*' => (1, 2),
        '+' => (3, 4),
        _ => panic!("Expected op, got: {}", c),
    });
    println!("Part 2: {}", sum);
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
