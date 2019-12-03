
enum WireDirection {
    UP, DOWN, LEFT, RIGHT
};

struct WireSegment {
    direction: WireDirection;
    length: usize;
};

type Wire = Vec<WireSegemtn>;

fn readWires(filename: &str) -> Vec<Wire> {
    let file = File::open("../input").unwrap();
    let reader = BufReader::new(&f);
}

fn main() {
    let file = File::open("../input").unwrap();
    let reader = BufReader::new(&f);
    for 
}
