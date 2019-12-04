use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fs::File;
use std::collections::HashMap;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_length(&self) -> u64 {
        return self.x.abs() as u64 + self.y.abs() as u64;
    }

    fn hash(&self) -> i64 {
        return self.x as i64 * 0x1000000 as i64 + self.y as i64;
    }
}

enum WireDirection {
    UP, DOWN, LEFT, RIGHT
}

struct WireSegment {
    direction: WireDirection,
    length: usize,
}

type Wire = Vec<WireSegment>;

struct WireIterator<'a> {
    wire: &'a Wire,
    segment: usize,
    segment_index: usize,
    position: Point,
}

impl WireIterator<'_> {
    fn new(wire: &Wire) -> WireIterator {
        return WireIterator { 
            wire: wire, 
            segment: 0, 
            segment_index: 0, 
            position: Point { x: 0, y: 0 },
        };
    }
}

impl Iterator for WireIterator<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        match self.wire[self.segment].direction {
            WireDirection::UP => self.position.y += 1,
            WireDirection::DOWN => self.position.y -= 1,
            WireDirection::LEFT => self.position.x -= 1,
            WireDirection::RIGHT => self.position.x += 1,
        }
        self.segment_index += 1;
        if self.segment_index >= self.wire[self.segment].length {
            self.segment += 1;
            self.segment_index = 0;
            if self.segment >= self.wire.len() {
                return None;
            }
        }
        return Some(Point {
            x: self.position.x, 
            y: self.position.y
        });
    }
}

fn wiresegment_from_str(s: &str) -> WireSegment {
    let direction = match &s[0..1] {
        "U" => WireDirection::UP,
        "D" => WireDirection::DOWN,
        "L" => WireDirection::LEFT,
        "R" => WireDirection::RIGHT,
        _ => panic!("Unknown direction")
    };
    return WireSegment {
        direction: direction,
        length: s[1..].parse::<usize>().unwrap(),
    };
}

fn read_wires(filename: &str) -> Vec<Wire> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);
    let mut wires: Vec<Wire> = Vec::new();
    for line in reader.lines() {
        let wire = line.unwrap().split(",").map(wiresegment_from_str).collect::<Wire>(); 
        wires.push(wire);
    }
    return wires;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let wires = read_wires(&args[1]);
    let mut pos_set: HashMap<i64, usize> = HashMap::new();
    for (i, point) in WireIterator::new(&wires[0]).enumerate() {
        let hash = point.hash();
        if !pos_set.contains_key(&hash) {
            pos_set.insert(point.hash(), i + 1);
        }
    }
    let mut min_dist = 0x1000000;
    let mut min_intersection = Point { x: 0, y: 0 };
    for (i, point) in WireIterator::new(&wires[1]).enumerate() {
        let hash = point.hash();
        if pos_set.contains_key(&hash) {
            //let dist = point.manhattan_length();
            let dist = (i + 1) + pos_set[&hash];
            println!("Intersection at {}, {}. dist = {}", point.x, point.y, dist);
            if dist < min_dist {
                min_intersection = point;
                min_dist = dist;
            }
        }
    }
    println!("Closest intersection at {}, {}. dist = {}", min_intersection.x, 
                                                          min_intersection.y, 
                                                          min_dist);
}
