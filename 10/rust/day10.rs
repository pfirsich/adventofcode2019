use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp;
use std::cmp::Ordering::Equal;
use std::f64::consts::{PI, FRAC_PI_2};

type BoolGrid = Vec<Vec<bool>>;

fn load_asteroid_map(filename: &str) -> BoolGrid {
    let file = BufReader::new(File::open(filename).expect("open failed"));
    let mut map: BoolGrid = Vec::new();
    for line in file.lines() {
        map.push(Vec::new());
        let last_idx = map.len() - 1;
        for c in line.expect("lines failed").chars() {
            map[last_idx].push(match c {
                '.' => false,
                '#' => true,
                _ => panic!("Unknown char")
            });
        }
        if map[last_idx].len() != map[0].len() {
            panic!("Non-rectangular map!");
        }
    }
    return map;
}

fn int_normalize(num: i64, denom: i64) -> (i64, i64) {
    assert!(num != 0 || denom != 0);
    let min = cmp::min(num.abs(), denom.abs());
    let max = cmp::max(num.abs(), denom.abs());
    if num == 0 || denom == 0 {
        return (num / max, denom / max);
    }
    let mut new_num = num;
    let mut new_denom = denom;
    for factor in 2..min+1 { // max is dumb and inefficient, but not trivially sufficient
        while new_num % factor == 0 && new_denom % factor == 0 {
            new_num /= factor;
            new_denom /= factor;
        }
    }
    return (new_num, new_denom);
}

fn get_visibility_map(asteroid_map: &BoolGrid, view_x: usize, view_y: usize) -> BoolGrid {
    let mut vis_map = BoolGrid::new();
    // Init map all visible
    for y in 0..asteroid_map.len() {
        let mut row: Vec<bool> = Vec::new();
        row.resize(asteroid_map[y].len(), true);
        vis_map.push(row);
    }

    // Find all obstacles
    for y in 0..asteroid_map.len() {
        for x in 0..asteroid_map[y].len() {
            if asteroid_map[y][x] && vis_map[y][x] && (x != view_x || y != view_y) { // If obstacle and still visible
                //println!("Start walk from {}, {}", x, y);
                // Walk along the line of sight and mark as not visible
                let rel_x = x as i64 - view_x as i64;
                let rel_y = y as i64 - view_y as i64;
                let (dir_x, dir_y) = int_normalize(rel_x, rel_y);
                assert!(dir_x != 0 || dir_y != 0);
                let mut cur_x = x;
                let mut cur_y = y;
                loop {
                    let next_x = cur_x as i64 + dir_x;
                    let next_y = cur_y as i64 + dir_y;
                    if next_x < 0 || next_x as usize >= asteroid_map[y].len() || next_y < 0 || next_y as usize >= asteroid_map.len() {
                        break;
                    }
                    cur_x = next_x as usize;
                    cur_y = next_y as usize;
                    //println!("Set invis {}, {}", cur_x, cur_y);
                    vis_map[cur_y][cur_x] = false;
                }
            }
        }
    }

    return vis_map;
}

fn get_visible_asteroids(asteroid_map: &BoolGrid, vis_map: &BoolGrid) -> Vec<(usize, usize)> {
    // I'm sure this can be done in some nice functional way or something
    let mut asteroids: Vec<(usize, usize)> = Vec::new();
    assert!(asteroid_map.len() == vis_map.len());
    for y in 0..asteroid_map.len() {
        assert!(asteroid_map[y].len() == vis_map[y].len());
        for x in 0..asteroid_map[y].len() {
            if asteroid_map[y][x] && vis_map[y][x] {
                asteroids.push((x, y));
            }
        }
    }
    return asteroids;
}

fn print_map(map: &BoolGrid, true_str: &str, false_str: &str) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", match map[y][x] {
                true => true_str,
                false => false_str,
            })
        }
        println!("");
    }
}

fn print_asteroids(asteroids: &Vec<(usize, usize)>) {
    let mut map: BoolGrid = BoolGrid::new();
    let mut size_x = 0;
    let mut size_y = 0;
    for asteroid in asteroids {
        size_x = cmp::max(size_x, asteroid.0 + 1);
        size_y = cmp::max(size_y, asteroid.1 + 1);
    }
    map.resize(size_y, Vec::new());
    for y in 0..size_y {
        map[y].resize(size_x, false);
    }
    for asteroid in asteroids {
        let (x, y) = asteroid;
        map[*y][*x] = true;
    }
    print_map(&map, "#", ".");
}

fn norm_angle(angle: f64) -> f64 {
    let mut a = angle;
    while a > 2.0 * PI {
        a -= 2.0 * PI;
    }
    while a < 0.0 {
        a += 2.0 * PI;
    }
    return a;
}

fn get_pos_angle(x: usize, y: usize, view_x: usize, view_y: usize) -> f64 {
    let rel_x = (x as i64 - view_x as i64) as f64;
    let rel_y = (y as i64 - view_y as i64) as f64;
    return norm_angle(rel_y.atan2(rel_x) + FRAC_PI_2);
}

fn main() {
    let map = load_asteroid_map("../input");
    println!("Asteroid map:");
    print_map(&map, "#", ".");

    let mut max_vis = 0;
    let mut max_vis_x = 0;
    let mut max_vis_y = 0;
    let (_x, _y) = int_normalize(-3, 9);
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] { // Asteroid
                let vis_map = get_visibility_map(&map, x, y);
                //print_map(&vis_map, " ", "X");
                let visible_count = get_visible_asteroids(&map, &vis_map).len() - 1; // -1 for OTHER asteroids
                if visible_count > max_vis {
                    max_vis = visible_count; 
                    max_vis_x = x;
                    max_vis_y = y;
                }
            }
        }
    }
    println!("Max {} asteroids visible from {}, {}", max_vis, max_vis_x, max_vis_y);
    
    let vis_map = get_visibility_map(&map, max_vis_x, max_vis_y);
    println!("Vis map:");
    print_map(&vis_map, " ", "X");
    
    let mut visible = get_visible_asteroids(&map, &vis_map);
    println!("Vaporized asteroids:");
    print_asteroids(&visible);
    visible.sort_by(|a, b| get_pos_angle(a.0, a.1, max_vis_x, max_vis_y).partial_cmp(&get_pos_angle(b.0, b.1, max_vis_x, max_vis_y)).unwrap_or(Equal));
    println!("in order: {:?}", visible);
    println!("1st: {:?}", visible[0]);
    println!("200th vaporized asteroid: {:?}", visible[199]);
    println!("201th vaporized asteroid: {:?}", visible[200]);
}