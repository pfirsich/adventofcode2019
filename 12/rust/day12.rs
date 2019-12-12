use std::ops::{Add, Sub};

fn sgn(x: i64) -> i64 {
    if x > 0 {
        return 1;
    } else if x < 0 {
        return -1;
    } else {
        return 0;
    }
}

#[derive(Clone, Copy)]
struct Vector {
    x: i64,
    y: i64,
    z: i64
}

impl Vector {
    fn sign(&self) -> Vector {
        return Vector {
            x: sgn(self.x),
            y: sgn(self.y),
            z: sgn(self.z)
        }
    }

    fn one_norm(&self) -> u64 {
        return (self.x.abs() + self.y.abs() + self.z.abs()) as u64;
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        return Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        };
    }
}

impl Sub for Vector {
    type Output = Vector;
    
    fn sub(self, rhs: Vector) -> Vector {
        return Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        };
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

fn step(new_state: &mut System, old_state: &System) {
    for i in 0..new_state.len() {
        new_state[i].velocity = old_state[i].velocity;
        for j in 0..new_state.len() {
            if i != j {
                let rel = old_state[j].position - old_state[i].position;
                new_state[i].velocity = new_state[i].velocity + rel.sign();
            }
        }
        new_state[i].position = old_state[i].position + new_state[i].velocity;
    }
}

fn total_energy(system: &System) -> u64 {
    let mut e: u64 = 0;
    for body in system {
        e += body.energy();
    }
    return e;
}

fn main() {
    let mut moons: System = vec![
        Body { position: Vector { x: 15, y: -2, z: -6 }, velocity: Vector::default() },
        Body { position: Vector { x: -5, y: -4, z: -11 }, velocity: Vector::default() },
        Body { position: Vector { x: 0, y: -6, z: 0 }, velocity: Vector::default() },
        Body { position: Vector { x: 5, y: 9, z: 6 }, velocity: Vector::default() },
    ];
    /*let mut moons: System = vec![
        Body { position: Vector { x: -1, y: 0, z: 2 }, velocity: Vector::default() },
        Body { position: Vector { x: 2, y: -10, z: -7 }, velocity: Vector::default() },
        Body { position: Vector { x: 4, y: -8, z: 8 }, velocity: Vector::default() },
        Body { position: Vector { x: 3, y: 5, z: -1 }, velocity: Vector::default() },
    ];*/
    let mut moons_new = moons.clone();
    for _step in 0..1000 {
        /*for moon in &moons {
            println!("pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>", 
                moon.position.x, moon.position.y, moon.position.z, 
                moon.velocity.x, moon.velocity.y, moon.velocity.z);
        }
        println!("");*/
        step(&mut moons_new, &moons);
        let temp = moons;
        moons = moons_new;
        moons_new = temp;
    }
    println!("Total energy: {}", total_energy(&moons));
}