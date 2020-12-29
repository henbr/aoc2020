use std::env;
use std::fs;

fn read(file: &str) -> Vec<Vec<char>> {
    let data = fs::read_to_string(file).unwrap();
    data.lines().map(|s| s.chars().collect()).collect()
}

fn seat_to_id(seat: &Vec<char>) -> u32 {
    let mut row = (0, 127);
    for c in seat.iter().take(7) {
        if *c == 'F' {
            row = (row.0, (row.0 + row.1) / 2);
        } else {
            row = ((row.0 + row.1) / 2 + 1, row.1);
        }
    }
    assert!(row.0 == row.1);

    let mut col = (0, 7);
    for c in seat.iter().skip(7) {
        if *c == 'L' {
            col = (col.0, (col.0 + col.1) / 2);
        } else {
            col = ((col.0 + col.1) / 2 + 1, col.1);
        }
    }
    assert!(col.0 == col.1);

    row.0 * 8 + col.0
}

fn get_highest_seat(file: &str) -> u32 {
    read(file)
        .iter()
        .map(seat_to_id)
        .fold(0, |sofar, id| sofar.max(id))
}

fn part1(file: &str) {
    let highest = get_highest_seat(file);
    println!("Part 1: {}", highest);
}

fn part2(file: &str) {
    let highest = get_highest_seat(file);
    let mut seats = vec![false; highest as usize + 1];
    read(file).iter().map(seat_to_id).for_each(|id| {
        seats[id as usize] = true;
    });
    let mut seat = -1;
    for idx in 1..seats.len() - 1 {
        let prev = seats[idx - 1];
        let next = seats[idx + 1];
        let current = seats[idx];
        if prev && next && !current {
            seat = idx as i32;
        }
    }
    println!("Part 2: {}", seat);
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
