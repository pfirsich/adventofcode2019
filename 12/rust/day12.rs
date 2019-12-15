use std::ops::{AddAssign};

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

    fn get_x(&self) -> i64 {
        return self.x;
    }

    fn get_y(&self) -> i64 {
        return self.y;
    }

    fn get_z(&self) -> i64 {
        return self.z;
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
    pos: Vector,
    vel: Vector,
}

impl Body {
    fn new(x: i64, y: i64, z: i64) -> Body {
        return Body { 
            pos: Vector { x: x, y: y, z: z },
            vel: Vector { x: 0, y: 0, z: 0 },
        };
    }

    fn energy(&self) -> u64 {
        return self.pos.one_norm() * self.vel.one_norm();
    }
}

type System = Vec<Body>;

fn step(state: &mut System) {
    for i in 0..state.len() {
        for j in 0..state.len() {
            if i != j {
                state[i].vel.x += (state[j].pos.x - state[i].pos.x).signum();
                state[i].vel.y += (state[j].pos.y - state[i].pos.y).signum();
                state[i].vel.z += (state[j].pos.z - state[i].pos.z).signum();
            }
        }
    }
    for body in state {
        body.pos += body.vel;
    }
}

fn state_equal(a: &System, b: &System, comp: fn(&Vector) -> i64) -> bool {
    assert!(a.len() == b.len());
    for i in 0..a.len() {
        if comp(&a[i].pos) != comp(&b[i].pos) || comp(&a[i].vel) != comp(&b[i].vel) {
            return false;
        }
    }
    return true;
}

fn main() {
    let start: System = vec![
        Body::new(15, -2, -6),
        Body::new(-5, -4, -11),
        Body::new(0, -6, 0),
        Body::new(5, 9, 6),
    ];
    /*let start: System = vec![
        Body::new(-1, 0, 2),
        Body::new(2, -10, -7),
        Body::new(4, -8, 8),
        Body::new(3, 5, -1),
    ];*/
    let mut moons = start.clone();
    for _step in 0..1000 {
        step(&mut moons);
    }
    let total_energy: u64 = moons.iter().map(Body::energy).sum();
    println!("Total energy: {}", total_energy);

    let mut steps: i64 = 0;
    let mut period: Vector = Vector::default();
    moons = start.clone();
    while period.x == 0 || period.y == 0 || period.z == 0 {
        step(&mut moons);
        steps += 1;
        if steps % 10000000 == 0 {
            println!("{} steps", steps);
        }

        if state_equal(&moons, &start, Vector::get_x) && period.x == 0 {
            period.x = steps;
            println!("Period x: {}", period.x);
        }
        if state_equal(&moons, &start, Vector::get_y) && period.y == 0 {
            period.y = steps;
            println!("Period y: {}", period.y);
        }
        if state_equal(&moons, &start, Vector::get_z) && period.z == 0 {
            period.z = steps;
            println!("Period z: {}", period.z);
        }
    }
    println!("https://www.wolframalpha.com/input/?i=lcm%28{}%2C{}%2C{}%29", period.x, period.y, period.z);
}