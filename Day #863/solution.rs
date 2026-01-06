// Day 863: generate() returns a finite but arbitrarily large binary tree in O(1).
// Approach: root created in O(1); children expanded lazily with randomized termination
// (each child exists with prob < 0.5 => branching process is finite almost surely).
// generate(): O(1). Materializing whole tree: O(size). Deterministic demo via MINSTD RNG.

const P: f64 = 0.45;
const DEPTH_CAP: i32 = 50;

struct Rng {
    state: i64,
}
impl Rng {
    fn next(&mut self) -> f64 {
        self.state = (self.state * 48271) % 2147483647;
        self.state as f64 / 2147483647.0
    }
}

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    expanded: bool,
}
impl Node {
    fn new() -> Box<Node> {
        Box::new(Node { left: None, right: None, expanded: false })
    }
}

fn ensure_children(n: &mut Node, depth: i32, rng: &mut Rng) {
    if n.expanded {
        return;
    }
    n.expanded = true;
    if depth >= DEPTH_CAP {
        return;
    }
    if rng.next() < P {
        n.left = Some(Node::new());
    }
    if rng.next() < P {
        n.right = Some(Node::new());
    }
}

fn generate() -> Box<Node> {
    Node::new()
} // O(1)

fn count_nodes(n: &mut Node, depth: i32, rng: &mut Rng) -> i32 {
    ensure_children(n, depth, rng);
    let mut total = 1;
    if let Some(l) = n.left.as_mut() {
        total += count_nodes(l, depth + 1, rng);
    }
    if let Some(r) = n.right.as_mut() {
        total += count_nodes(r, depth + 1, rng);
    }
    total
}

fn main() {
    let mut rng = Rng { state: 42 };
    let mut root = generate();
    println!(
        "Generated a finite binary tree with {} nodes (deterministic demo).",
        count_nodes(&mut root, 0, &mut rng)
    );
}
