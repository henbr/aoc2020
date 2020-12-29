use std::collections::HashSet;
use std::env;
use std::fs;

struct Pocket {
    map: HashSet<(i32, i32, i32, i32)>,
}

impl Pocket {
    fn new() -> Self {
        Self {
            map: HashSet::new(),
        }
    }

    fn activate(&mut self, x: i32, y: i32, z: i32, w: i32) {
        self.map.insert((x, y, z, w));
    }

    fn get_status(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        self.map.contains(&(x, y, z, w))
    }

    fn get_active(&self) -> Vec<(i32, i32, i32, i32)> {
        self.map.iter().map(|c| c.clone()).collect()
    }

    fn get_num_active(&self) -> usize {
        self.get_active().len()
    }

    fn get_range(&self) -> ((i32, i32, i32, i32), (i32, i32, i32, i32)) {
        let active = self.get_active();
        if active.len() == 0 {
            panic!("No active cells");
        }
        let mut minx = active[0].0;
        let mut miny = active[0].1;
        let mut minz = active[0].2;
        let mut minw = active[0].3;
        let mut maxx = active[0].0;
        let mut maxy = active[0].1;
        let mut maxz = active[0].2;
        let mut maxw = active[0].3;
        for a in active.iter() {
            minx = minx.min(a.0);
            miny = miny.min(a.1);
            minz = minz.min(a.2);
            minw = minw.min(a.3);
            maxx = maxx.max(a.0);
            maxy = maxy.max(a.1);
            maxz = maxz.max(a.2);
            maxw = maxw.max(a.3);
        }
        (
            (minx - 1, miny - 1, minz - 1, minw - 1),
            (maxx + 1, maxy + 1, maxz + 1, maxw + 1),
        )
    }

    fn calculate_cube(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        let mut active_count = 0;
        for nw in (w - 1)..(w + 2) {
            for nz in (z - 1)..(z + 2) {
                for ny in (y - 1)..(y + 2) {
                    for nx in (x - 1)..(x + 2) {
                        if nx == x && ny == y && nz == z && nw == w {
                            continue;
                        }
                        if self.get_status(nx, ny, nz, nw) {
                            active_count += 1;
                        }
                    }
                }
            }
        }
        /*
        If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
        If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
        */
        let status = self.get_status(x, y, z, w);
        if status && (active_count == 2 || active_count == 3) {
            true
        } else if !status && active_count == 3 {
            true
        } else {
            false
        }
    }

    fn step(&mut self, simulate_w: bool) {
        let mut new_map = HashSet::new();
        let (min, max) = self.get_range();
        let (minw, maxw) = if simulate_w { (min.3, max.3) } else { (0, 0) };
        let mut count = 0;
        for w in minw..maxw + 1 {
            for z in min.2..max.2 + 1 {
                for y in min.1..max.1 + 1 {
                    for x in min.0..max.0 + 1 {
                        count += 1;
                        if self.calculate_cube(x, y, z, w) {
                            new_map.insert((x, y, z, w));
                        }
                    }
                }
            }
        }
        println!("Cube count: {}", count);
        self.map = new_map;
    }
}

fn load_pocket(file: &str) -> Pocket {
    let data = fs::read_to_string(file).unwrap();
    let mut pocket = Pocket::new();
    for (y, l) in data.lines().map(|line| line.chars()).enumerate() {
        for (x, c) in l.enumerate() {
            if c == '#' {
                pocket.activate(x as i32, y as i32, 0, 0);
            }
        }
    }
    pocket
}

fn part1(file: &str) {
    let mut pocket = load_pocket(file);
    for _ in 0..6 {
        pocket.step(false);
    }
    println!("Part 1: num active: {}", pocket.get_num_active());
}

fn part2(file: &str) {
    let mut pocket = load_pocket(file);
    for _ in 0..6 {
        pocket.step(true);
    }
    println!("Part 2: num active: {}", pocket.get_num_active());
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
