use std::fs::File;
use std::io::{BufRead, BufReader};

const CONSIDER_FUEL_WEIGHT: bool = true;

fn get_fuel(mass: u32) -> u32 {
    let fuel = mass as i32 / 3 - 2;
    return if fuel > 0 { fuel as u32 } else { 0 };
}

fn get_fuel_for_fuel(fuel: u32) -> u32 {
    let mut total_fuel = 0;
    let mut extra_fuel = fuel;
    loop {
        extra_fuel = get_fuel(extra_fuel);
        if extra_fuel > 0 {
            total_fuel += extra_fuel;
        } else {
            return total_fuel;
        }
    }
}

fn main() {
    let file = File::open("../input").unwrap();
    let reader = BufReader::new(file);

    let mut total_fuel = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mass = line.parse::<u32>().unwrap();
        let fuel = get_fuel(mass);
        total_fuel += fuel;
        if CONSIDER_FUEL_WEIGHT {
            total_fuel += get_fuel_for_fuel(fuel);
        }
    }

    println!("Total fuel: {}", total_fuel);
}
