use std::collections::HashMap;
use std::env;
use std::fs;

fn read(file: &str) -> Vec<Vec<char>> {
    let data = fs::read_to_string(file).unwrap();
    data.lines().map(|s| s.chars().collect()).collect()
}

struct Answers {
    map: HashMap<char, u32>,
    num_people: u32,
}

impl Answers {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            num_people: 0,
        }
    }

    fn add_answers(&mut self, new_answers: &Vec<char>) {
        new_answers
            .iter()
            .for_each(|yes| *self.map.entry(*yes).or_insert(0) += 1);
        self.num_people += 1;
    }

    fn get_count(&self) -> usize {
        self.map.keys().len()
    }

    fn get_all_count(&self) -> usize {
        let mut sum = 0;
        for (_, num) in self.map.iter() {
            if *num == self.num_people {
                sum += 1;
            }
        }
        sum
    }
}

fn part1(file: &str) {
    let lines = read(file);
    let mut sum = 0;
    let mut answers = Answers::new();
    for line in lines.iter() {
        if line.is_empty() {
            sum += answers.get_count();
            answers = Answers::new();
        } else {
            answers.add_answers(line);
        }
    }
    sum += answers.get_count();
    println!("Part 1: {}", sum);
}

fn part2(file: &str) {
    let lines = read(file);
    let mut sum = 0;
    let mut answers = Answers::new();
    for line in lines.iter() {
        if line.is_empty() {
            sum += answers.get_all_count();
            answers = Answers::new();
        } else {
            answers.add_answers(line);
        }
    }
    sum += answers.get_all_count();
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
