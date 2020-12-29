use std::collections::HashMap;
use std::time::Instant;

/*
$ cargo run
Part1: 260
Part2 (map): 950 (15469ms)
Part2 (vec): 950 (2192ms)

$ cargo run --release
Part1: 260
Part2 (map): 950 (1456ms)
Part2 (vec): 950 (416ms)
*/

const INPUT: &str = "13,0,10,12,1,5,8";

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

fn play_map(numbers: &Vec<u32>, turns: u32) -> u32 {
    let mut previous_turns = numbers
        .iter()
        .enumerate()
        .map(|(idx, n)| (*n, (idx + 1) as u32))
        .collect::<HashMap<u32, u32>>();
    let mut last_number = *numbers.last().unwrap();
    for last_turn in (numbers.len() as u32)..turns {
        let previous_turn = previous_turns.insert(last_number, last_turn);
        last_number = if let Some(previous_turn) = previous_turn {
            last_turn - previous_turn
        } else {
            0
        };
    }
    last_number
}

fn play_vec(numbers: &Vec<u32>, turns: u32) -> u32 {
    let mut previous_turns = vec![0; turns as usize];
    numbers
        .iter()
        .enumerate()
        .for_each(|(idx, n)| previous_turns[*n as usize] = (idx + 1) as u32);
    let mut last_number = *numbers.last().unwrap();
    for last_turn in (numbers.len() as u32)..turns {
        let previous_turn = previous_turns[last_number as usize];
        previous_turns[last_number as usize] = last_turn;
        last_number = if previous_turn != 0 {
            last_turn - previous_turn
        } else {
            0
        };
    }
    last_number
}

fn part1(input: &str) {
    let numbers = parse_numbers(input);
    let ans = play_map(&numbers, 2020);
    println!("Part1: {}", ans);
}

fn part2(input: &str) {
    let numbers = parse_numbers(input);
    let instant = Instant::now();
    let ans = play_map(&numbers, 30000000);
    println!("Part2 (map): {} ({}ms)", ans, instant.elapsed().as_millis());
    let instant = Instant::now();
    let ans = play_vec(&numbers, 30000000);
    println!("Part2 (vec): {} ({}ms)", ans, instant.elapsed().as_millis());
}

fn main() {
    part1(INPUT);
    part2(INPUT);
}
