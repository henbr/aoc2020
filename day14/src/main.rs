use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

lazy_static! {
    static ref MEM_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$",).unwrap();
    static ref MASK_RE: Regex = Regex::new(r"^mask = (.+)$",).unwrap();
}

enum Command {
    Mem(u64, u64),  // address, value
    Mask(u64, u64), // mem write mask, value
}

fn parse_line(line: &str) -> Command {
    if line.starts_with("mem") {
        if let Some(caps) = MEM_RE.captures(line) {
            Command::Mem(
                caps[1].parse::<u64>().unwrap(),
                caps[2].parse::<u64>().unwrap(),
            )
        } else {
            panic!("Failed parsing mem: {}", line);
        }
    } else if line.starts_with("mask") {
        if let Some(caps) = MASK_RE.captures(line) {
            let mask = caps[1]
                .chars()
                .fold(0, |mask, c| (mask << 1) | (if c == 'X' { 1 } else { 0 }));
            let value = caps[1]
                .chars()
                .fold(0, |mask, c| (mask << 1) | (if c == '1' { 1 } else { 0 }));
            Command::Mask(mask, value)
        } else {
            panic!("Failed parsing mask: {}", line);
        }
    } else {
        panic!("Input error: {}", line);
    }
}

fn read_commands(file_name: &str) -> Vec<Command> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(parse_line)
        .collect()
}

fn part1(file_name: &str) {
    let cmds = read_commands(file_name);
    let mut memory = HashMap::new();
    let mut write_mask: u64 = 0;
    let mut mask_value: u64 = 0;
    for cmd in cmds.iter() {
        match cmd {
            Command::Mask(mask, value) => {
                write_mask = *mask;
                mask_value = *value;
            }
            Command::Mem(address, value) => {
                let new_value = (value & write_mask) | mask_value;
                memory.insert(address, new_value);
            }
        }
    }
    let sum = memory.values().fold(0, |acc, v| acc + v);
    println!("Part1: {}", sum);
}

fn part2(file_name: &str) {
    let cmds = read_commands(file_name);
    let mut memory = HashMap::<u64, u64>::new();
    let mut mask_floating: u64 = 0;
    let mut one_mask: u64 = 0;
    for cmd in cmds.iter() {
        match cmd {
            Command::Mask(mask, value) => {
                mask_floating = *mask;
                one_mask = *value;
            }
            Command::Mem(address, value) => {
                let num_variations = 2u64.pow(mask_floating.count_ones());
                // Calculate the floating address variations by distributing the
                // counter bits where the address mask is 'X'
                for counter in 0..num_variations {
                    let mut counter_bit_mask = 1;
                    let mut floating_bits = 0;
                    for n in (0..36).rev() {
                        floating_bits = floating_bits << 1;
                        if mask_floating & (1 << n) != 0 {
                            if counter & counter_bit_mask != 0 {
                                floating_bits |= 1;
                            }
                            counter_bit_mask <<= 1;
                        }
                    }
                    let new_address = ((address | one_mask) & !mask_floating) | floating_bits;
                    memory.insert(new_address, *value);
                }
            }
        }
    }
    let sum = memory.values().fold(0, |acc, v| acc + v);
    println!("Part2: {}", sum);
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
