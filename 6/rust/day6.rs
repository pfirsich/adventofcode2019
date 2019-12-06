use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn read_orbit_map(filename: &str) -> HashMap<String, String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);
    let mut map: HashMap<String, String> = HashMap::new();
    for line in reader.lines() {
    	let line_str: &String = &line.unwrap();
        let mut split = line_str.splitn(2, ")");
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        map.insert(second.to_string(), first.to_string());
    }
    return map;
}

fn walk_orbit_chain(orbits: &HashMap<String, String>, object: &String, mut chain: &mut Vec<String>) {
    if orbits.contains_key(object) {
        chain.push(orbits[object].clone());
        walk_orbit_chain(orbits, &orbits[object], &mut chain);
    }
}

fn get_orbit_chain(orbits: &HashMap<String, String>, object: &String) -> Vec<String> {
    let mut chain: Vec<String> = Vec::new();
    walk_orbit_chain(orbits, object, &mut chain);
    return chain;
}

fn get_first_common_object(objectA: &String, objectB: &String) -> String {
    let chainA = get_orbit_chain(orbits, objectA);
    let chainB = get_orbit_chain(orbits, objectB);
    for (i, object) in chainA.iter().enumerate() {
        match chainB.iter().position(|&x| x == object) {
            Some(common) => return ,
            None => ()
        }
    }
}

fn main() {
	let orbits = read_orbit_map("../input");
	let mut count = 0;
	for (object, _) in &orbits {
        count += get_orbit_chain(&orbits, &object).len();
	}
    let common = get_first_common_object(&orbits, "YOU", "SAN");
	println!("Total orbits: {}", count);
}