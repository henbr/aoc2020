use std::{env, fs};

fn read_joltage_adapters(file_name: &str) -> Vec<u32> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn part1(file_name: &str) {
    let mut adapters = read_joltage_adapters(file_name);

    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);

    let mut diffs1 = 0;
    let mut diffs3 = 0;
    let mut last_adapter = 0;
    for a in adapters.iter() {
        let diff = *a - last_adapter;
        last_adapter = *a;
        if diff == 1 {
            diffs1 += 1;
        }
        if diff == 3 {
            diffs3 += 1;
        }
    }
    println!("Part1: {}*{}={}", diffs1, diffs3, diffs1 * diffs3);
}

fn part2(file_name: &str) {
    let mut adapters = read_joltage_adapters(file_name);
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);

    let mut arrangements = vec![1];
    for i in 1..adapters.len() {
        let mut sum = 0u64;
        for j in (0..i).rev() {
            if adapters[i] - adapters[j] > 3 {
                break;
            }
            sum += arrangements[j];
        }
        arrangements.push(sum);
    }

    println!("Part2: {}", arrangements.last().unwrap());
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
