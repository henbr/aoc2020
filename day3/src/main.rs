use std::env;
use std::fs;

struct Map {
    lines: Vec<Vec<char>>,
}

impl Map {
    fn new() -> Self {
        Self { lines: Vec::new() }
    }

    fn add(&mut self, line: Vec<char>) {
        self.lines.push(line);
    }

    fn is_tree(&self, x: i32, y: i32) -> Option<bool> {
        if x >= 0 && y >= 0 && (y as usize) < self.lines.len() {
            let line = &self.lines[y as usize];
            let width = line.len();
            let pos = x as usize % width;
            Some(line[pos] == '#')
        } else {
            None
        }
    }
}

fn read_map(file: &str) -> Map {
    let data = fs::read_to_string(file).unwrap();
    let mut map = Map::new();
    for l in data.lines() {
        map.add(l.chars().collect());
    }
    map
}

fn count_trees(map: &Map, slopex: i32, slopey: i32) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    loop {
        let is_tree = map.is_tree(x, y);
        if let Some(is_tree) = is_tree {
            if is_tree {
                count += 1;
            }
        } else {
            break;
        }
        x += slopex;
        y += slopey;
    }
    count as u64
}

fn part1(file: &str) {
    let map = read_map(file);
    let count = count_trees(&map, 3, 1);
    println!("Part 1: number of trees: {}", count);
}

fn part2(file: &str) {
    let map = read_map(file);
    let counts = vec![
        count_trees(&map, 1, 1),
        count_trees(&map, 3, 1),
        count_trees(&map, 5, 1),
        count_trees(&map, 7, 1),
        count_trees(&map, 1, 2),
    ];
    let count = counts.iter().fold(1, |sofar, c| sofar * c);
    println!("Part 2: result: {}", count);
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
