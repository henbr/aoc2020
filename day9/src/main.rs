use std::env;
use std::fs;

//const PREAMBLE: usize = 5; // example.txt

const PREAMBLE: usize = 25; // input.txt

const BUFSIZE: usize = PREAMBLE + 1;

type Buffer = [u64; BUFSIZE];

fn read_numbers(file_name: &str) -> Vec<u64> {
    let numbers = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    if numbers.len() <= BUFSIZE {
        panic!("Too big BUFSIZE for input.");
    }
    numbers
}

fn add_to_buffer(buffer: &mut Buffer, num: u64) {
    for i in 0..buffer.len() - 1 {
        buffer[i] = buffer[i + 1];
    }
    buffer[buffer.len() - 1] = num;
}

fn is_last_valid(buffer: &Buffer) -> bool {
    let last = buffer[buffer.len() - 1];
    for i in 0..buffer.len() - 2 {
        let numi = buffer[i];
        if numi > last {
            continue;
        }
        let to_be_found = last - numi;
        for j in i + 1..buffer.len() - 1 {
            if buffer[j] == to_be_found {
                return true;
            }
        }
    }
    false
}

fn find_first_invalid(numbers: &Vec<u64>) -> u64 {
    let mut buffer = [0; BUFSIZE];
    for i in 0..BUFSIZE {
        add_to_buffer(&mut buffer, numbers[i]);
    }
    let mut first_invalid = None;
    for i in BUFSIZE..numbers.len() {
        let num = numbers[i];
        add_to_buffer(&mut buffer, num);
        if !is_last_valid(&buffer) {
            first_invalid = Some(num);
            break;
        }
    }
    if let Some(first_invalid) = first_invalid {
        first_invalid
    } else {
        panic!("First invalid not found");
    }
}

// Exclusive end
fn find_sum_range(numbers: &Vec<u64>, num_to_find: u64) -> (usize, usize) {
    let mut range = None;
    'outer: for start in 0..numbers.len() {
        let mut sum = 0;
        for end in start..numbers.len() {
            sum += numbers[end];
            if sum > num_to_find {
                break;
            } else if sum == num_to_find {
                range = Some((start, end + 1));
                break 'outer;
            }
        }
    }
    range.unwrap()
}

fn part1(file_name: &str) {
    let numbers = read_numbers(file_name);
    let first_invalid = find_first_invalid(&numbers);
    println!("Part1: {}", first_invalid);
}

fn part2(file_name: &str) {
    let numbers = read_numbers(file_name);
    let first_invalid = find_first_invalid(&numbers);
    let range = find_sum_range(&numbers, first_invalid);
    let mut min = numbers[range.0];
    let mut max = numbers[range.0];
    for i in range.0..range.1 {
        min = min.min(numbers[i]);
        max = max.max(numbers[i]);
    }
    println!("Part2: {}", min + max);
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
