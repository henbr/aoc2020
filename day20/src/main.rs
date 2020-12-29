use std::{env, fs};

const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

// Stores edges as 10-bit integers. It wasn't as simple as it first seemed. It
// had probably been better to rotate/mirror the tile data instead, especially
// since this had to be done for part 2 anyway.
struct Tile {
    number: u64,
    edges: [[u32; 4]; 8],
    data: Vec<char>,
}

struct Placement {
    tile: usize,
    orientation: usize,
    x: i32,
    y: i32,
}

fn parse_edges(lines: &[&str], mirror: bool) -> [u32; 4] {
    let lines = lines
        .iter()
        .skip(1) // tile number row
        .map(|l| {
            if mirror {
                l.chars().rev().collect()
            } else {
                l.chars().collect()
            }
        })
        .collect::<Vec<Vec<char>>>();
    let mut t = 0;
    let mut b = 0;
    let mut l = 0;
    let mut r = 0;
    for i in 0..10 {
        t = (t << 1) | if lines[0][i] == '#' { 1 } else { 0 };
        r = (r << 1) | if lines[i][9] == '#' { 1 } else { 0 };
        b = (b << 1) | if lines[9][9 - i] == '#' { 1 } else { 0 };
        l = (l << 1) | if lines[9 - i][0] == '#' { 1 } else { 0 };
    }
    [t, r, b, l]
}

fn rotate_tile(edges: &[u32; 4], num_rotates: u32) -> [u32; 4] {
    if num_rotates == 0 {
        edges.clone()
    } else {
        rotate_tile(
            &[edges[LEFT], edges[TOP], edges[RIGHT], edges[BOTTOM]],
            num_rotates - 1,
        )
    }
}

fn flip_bl_edges(edges: &[u32; 4]) -> [u32; 4] {
    fn flip(edge: u32) -> u32 {
        let mut flipped = 0;
        for i in 0..10 {
            let bit = (edge & (1 << i)) >> i;
            flipped = (flipped << 1) | bit;
        }
        flipped
    }
    [
        edges[TOP],
        edges[RIGHT],
        flip(edges[BOTTOM]),
        flip(edges[LEFT]),
    ]
}

fn parse_tile(lines: &[&str]) -> Tile {
    let number = lines[0][5..9].parse::<u64>().unwrap();
    let edges = parse_edges(lines, false);
    let edgesm = parse_edges(lines, true);
    let all_edges = [
        flip_bl_edges(&edges),
        flip_bl_edges(&rotate_tile(&edges, 1)),
        flip_bl_edges(&rotate_tile(&edges, 2)),
        flip_bl_edges(&rotate_tile(&edges, 3)),
        flip_bl_edges(&edgesm),
        flip_bl_edges(&rotate_tile(&edgesm, 1)),
        flip_bl_edges(&rotate_tile(&edgesm, 2)),
        flip_bl_edges(&rotate_tile(&edgesm, 3)),
    ];
    let data = lines[1..].join("").chars().collect::<Vec<char>>();
    Tile {
        number,
        edges: all_edges,
        data,
    }
}

fn read_tiles(file_name: &str) -> Vec<Tile> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()
        .chunks(12)
        .map(|c| parse_tile(c))
        .collect()
}

fn get_placement(placements: &Vec<Placement>, x: i32, y: i32) -> Option<&Placement> {
    for p in placements {
        if p.x == x && p.y == y {
            return Some(p);
        }
    }
    None
}

fn edges_match(tiles: &Vec<Tile>, p1: &Placement, p2: &Placement, e1: usize, e2: usize) -> bool {
    let edges1 = &tiles[p1.tile].edges[p1.orientation];
    let edges2 = &tiles[p2.tile].edges[p2.orientation];
    edges1[e1] == edges2[e2]
}

fn placement_ok(tiles: &Vec<Tile>, placements: &Vec<Placement>, placement: &Placement) -> bool {
    if let Some(p) = get_placement(placements, placement.x - 1, placement.y) {
        if !edges_match(tiles, p, placement, RIGHT, LEFT) {
            return false;
        }
    }
    if let Some(p) = get_placement(placements, placement.x + 1, placement.y) {
        if !edges_match(tiles, p, placement, LEFT, RIGHT) {
            return false;
        }
    }
    if let Some(p) = get_placement(placements, placement.x, placement.y - 1) {
        if !edges_match(tiles, p, placement, BOTTOM, TOP) {
            return false;
        }
    }
    if let Some(p) = get_placement(placements, placement.x, placement.y + 1) {
        if !edges_match(tiles, p, placement, TOP, BOTTOM) {
            return false;
        }
    }
    true
}

fn next_position(num_tiles: usize, x: i32, y: i32) -> (i32, i32) {
    let size = (num_tiles as f32).sqrt() as i32;
    ((x + 1) % size, y + (x + 1) / size)
}

fn place_tiles(tiles: &Vec<Tile>, placements: &mut Vec<Placement>, x: i32, y: i32) -> bool {
    if tiles.len() == placements.len() {
        return true;
    }
    for tile in 0..tiles.len() {
        if placements.iter().position(|t| t.tile == tile).is_some() {
            continue;
        }
        for orientation in 0..8 {
            let placement = Placement {
                tile,
                orientation,
                x,
                y,
            };
            if placement_ok(tiles, placements, &placement) {
                placements.push(placement);
                let (nx, ny) = next_position(tiles.len(), x, y);
                if place_tiles(tiles, placements, nx, ny) {
                    return true;
                }
                placements.pop();
            }
        }
    }
    false
}

fn assemble_image(tiles: &Vec<Tile>, placements: &Vec<Placement>) -> Vec<char> {
    let size = (tiles.len() as f32).sqrt() as usize;
    let mut image = vec!['.'; size * size * 8 * 8];
    let image_width = size * 8;
    for y in 0..size {
        for x in 0..size {
            if let Some(p) = get_placement(placements, x as i32, y as i32) {
                let t = &tiles[p.tile];
                let tlx = x * 8;
                let tly = y * 8;
                for ty in 0..8 {
                    for tx in 0..8 {
                        let (tix, tiy) = transform_coords(
                            8,
                            tx,
                            ty,
                            (p.orientation % 4) as u32,
                            p.orientation >= 4,
                        );
                        let ioffset = (tly + tiy as usize) * image_width + tlx + tix as usize;
                        let toffset = ((ty + 1) * 10 + (tx + 1)) as usize;
                        image[ioffset] = t.data[toffset];
                    }
                }
            }
        }
    }
    image
}

fn image_size(image: &Vec<char>) -> usize {
    (image.len() as f32).sqrt() as usize
}

fn transform_coords(size: i32, x: i32, y: i32, rotates: u32, mirror: bool) -> (i32, i32) {
    let size = size - 1;
    let x = if mirror { size - x } else { x };
    match rotates {
        0 => (x, y),
        1 => (size - y, x),
        2 => (size - x, size - y),
        3 => (y, size - x),
        _ => panic!("wrong number of roates"),
    }
}

fn get_pixel(image: &Vec<char>, x: i32, y: i32, rotates: u32, mirror: bool) -> char {
    let size = image_size(image) as i32;
    let (x, y) = transform_coords(size, x, y, rotates, mirror);
    image[(y * size + x) as usize]
}

fn set_pixel(image: &mut Vec<char>, x: i32, y: i32, rotates: u32, mirror: bool, new_value: char) {
    let size = image_size(image) as i32;
    let (x, y) = transform_coords(size, x, y, rotates, mirror);
    image[(y * size + x) as usize] = new_value;
}

fn mark_sea_monster(
    image: &mut Vec<char>,
    monster: &Vec<char>,
    monster_width: usize,
    monster_height: usize,
    rotates: u32,
    mirror: bool,
    x: i32,
    y: i32,
) {
    for my in 0..monster_height {
        for mx in 0..monster_width {
            let ix = x + mx as i32;
            let iy = y + my as i32;
            let pixel = get_pixel(image, ix, iy, rotates, mirror);
            let mpixel = monster[my * monster_width + mx];
            if mpixel == '#' && !(pixel == '#') {
                return;
            }
        }
    }
    for my in 0..monster_height {
        for mx in 0..monster_width {
            let mpixel = monster[my * monster_width + mx];
            if mpixel == '#' {
                let ix = x + mx as i32;
                let iy = y + my as i32;
                set_pixel(image, ix, iy, rotates, mirror, 'O');
            }
        }
    }
}

fn mark_sea_monsters(image: &mut Vec<char>) {
    let size = (image.len() as f32).sqrt() as i32;
    let monster = "                  # #    ##    ##    ### #  #  #  #  #  #   "
        .chars()
        .collect::<Vec<char>>();
    let monster_height: usize = 3;
    let monster_width = monster.len() / monster_height;

    for mirror in &[false, true] {
        for rotates in 0..4 {
            for y in 0..size - monster_height as i32 {
                for x in 0..size - monster_width as i32 {
                    mark_sea_monster(
                        image,
                        &monster,
                        monster_width,
                        monster_height,
                        rotates,
                        *mirror,
                        x,
                        y,
                    );
                }
            }
        }
    }
}

fn part1(file_name: &str) {
    let tiles = read_tiles(file_name);
    let mut placements = Vec::new();
    if place_tiles(&tiles, &mut placements, 0, 0) {
        let size = (tiles.len() as f32).sqrt() as usize;
        let ans = tiles[placements[0].tile].number
            * tiles[placements[size - 1].tile].number
            * tiles[placements[size * (size - 1)].tile].number
            * tiles[placements[size * size - 1].tile].number;
        println!("Part1: {}", ans);
    } else {
        println!("failed");
    }
}

/*
fn debug_draw_image(image: &Vec<char>) {
    let size = image_size(&image);
    image.chunks(size).for_each(|l| {
        l.iter().for_each(|&c| {
            if c == '#' {
                print!("██");
            } else if c == 'O' {
                //print!("░░");
                print!("🐊");
            } else {
                print!("  ")
            }
        });
        println!("");
    });
} */

fn part2(file_name: &str) {
    let tiles = read_tiles(file_name);
    let mut placements = Vec::new();
    if place_tiles(&tiles, &mut placements, 0, 0) {
        let mut image = assemble_image(&tiles, &placements);
        mark_sea_monsters(&mut image);
        let ans = image
            .iter()
            .fold(0, |acc, c| if *c == '#' { acc + 1 } else { acc });
        println!("Part2: {}", ans);
        // debug_draw_image(&image);
    } else {
        println!("failed");
    }
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
