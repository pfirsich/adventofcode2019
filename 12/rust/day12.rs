use std::ops::{Add, AddAssign, Sub};

fn sgn(x: i64) -> i64 {
    if x > 0 {
        return 1;
    } else if x < 0 {
        return -1;
    } else {
        return 0;
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Vector {
    x: i64,
    y: i64,
    z: i64
}

impl Vector {
    fn one_norm(&self) -> u64 {
        return (self.x.abs() + self.y.abs() + self.z.abs()) as u64;
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Default for Vector {
    fn default() -> Self {
        return Vector { x: 0, y: 0, z: 0 }
    }
}

#[derive(Clone)]
struct Body {
    position: Vector,
    velocity: Vector,
}

impl Body {
    fn energy(&self) -> u64 {
        return self.position.one_norm() * self.velocity.one_norm();
    }
}

type System = Vec<Body>;

fn step(state: &mut System) {
    for i in 0..state.len() {
        for j in 0..state.len() {
            if i != j {
                state[i].velocity.x += sgn(state[j].position.x - state[i].position.x);
                state[i].velocity.y += sgn(state[j].position.y - state[i].position.y);
                state[i].velocity.z += sgn(state[j].position.z - state[i].position.z);
            }
        }
    }
    for body in state {
        body.position += body.velocity;
    }
}

fn total_energy(system: &System) -> u64 {
    let mut e: u64 = 0;
    for body in system {
        e += body.energy();
    }
    return e;
}

fn state_equal(a: &System, b: &System) -> bool {
    assert!(a.len() == b.len());
    for i in 0..a.len() {
        if a[i].position != b[i].position || a[i].velocity != b[i].velocity {
            return false;
        }
    }
    return true;
}

fn main() {
    let start: System = vec![
        Body { position: Vector { x: 15, y: -2, z: -6 }, velocity: Vector::default() },
        Body { position: Vector { x: -5, y: -4, z: -11 }, velocity: Vector::default() },
        Body { position: Vector { x: 0, y: -6, z: 0 }, velocity: Vector::default() },
        Body { position: Vector { x: 5, y: 9, z: 6 }, velocity: Vector::default() },
    ];
    /*let start: System = vec![
        Body { position: Vector { x: -1, y: 0, z: 2 }, velocity: Vector::default() },
        Body { position: Vector { x: 2, y: -10, z: -7 }, velocity: Vector::default() },
        Body { position: Vector { x: 4, y: -8, z: 8 }, velocity: Vector::default() },
        Body { position: Vector { x: 3, y: 5, z: -1 }, velocity: Vector::default() },
    ];*/
    let mut moons = start.clone();
    for _step in 0..1000 {
        step(&mut moons);
    }
    println!("Total energy: {}", total_energy(&moons));

    let mut steps = 0;
    loop {
        step(&mut moons);
        steps += 1;
        if steps % 10000000 == 0 {
            println!("{} steps", steps);
        }

        if state_equal(&moons, &start) {
            break;
        }
    }
    println!("State back to start after {} steps", steps);
}