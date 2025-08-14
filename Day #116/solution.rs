// Day 116: generate() returns a root in O(1); children materialize lazily on access.
// Each node spawns children with a depth-decaying probability => finite a.s. but unbounded.
// (Demo uses a seeded Park-Miller LCG so the forced size is reproducible.)
struct Node {
    depth: i32,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

struct Rng {
    state: i64,
}
impl Rng {
    fn next(&mut self) -> i64 {
        self.state = (self.state * 16807) % 2147483647;
        self.state
    }
}

fn threshold(d: i32) -> i64 {
    let t = 750 - 80 * d as i64;
    if t > 0 { t } else { 0 }
}

fn generate() -> Box<Node> {
    Box::new(Node { depth: 0, l: None, r: None }) // O(1)
}

fn force(n: &mut Node, rng: &mut Rng) -> i32 {
    let mut cnt = 1;
    if rng.next() % 1000 < threshold(n.depth) {
        let mut child = Box::new(Node { depth: n.depth + 1, l: None, r: None });
        cnt += force(&mut child, rng);
        n.l = Some(child);
    }
    if rng.next() % 1000 < threshold(n.depth) {
        let mut child = Box::new(Node { depth: n.depth + 1, l: None, r: None });
        cnt += force(&mut child, rng);
        n.r = Some(child);
    }
    cnt
}

fn main() {
    let mut rng = Rng { state: 42 };
    let mut root = generate();
    let n = force(&mut root, &mut rng);
    println!("Generated a finite binary tree with {} nodes", n);
}
