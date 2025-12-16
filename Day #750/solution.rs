// generate() returns a root in O(1); children are materialized lazily on first access.
// Each child exists with probability p<0.5, so the tree is finite (a.s.) yet unbounded.
// generate(): O(1). Traversal materializes nodes on demand.
use std::cell::RefCell;

const P: f64 = 0.45;

// Simple seeded xorshift64 RNG (std has no RNG).
struct Rng(u64);
impl Rng {
    fn next_f(&mut self) -> f64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x >> 11) as f64 / (1u64 << 53) as f64
    }
}

enum Child {
    Pending,
    Done(Option<Box<Node>>),
}

struct Node {
    #[allow(dead_code)]
    value: i32,
    left: RefCell<Child>,
    right: RefCell<Child>,
}

fn new_node() -> Node {
    Node {
        value: 0,
        left: RefCell::new(Child::Pending),
        right: RefCell::new(Child::Pending),
    }
}

fn generate() -> Node {
    new_node() // O(1)
}

fn force(cell: &RefCell<Child>, rng: &mut Rng) {
    let pending = matches!(*cell.borrow(), Child::Pending);
    if pending {
        let child = if rng.next_f() < P {
            Child::Done(Some(Box::new(new_node())))
        } else {
            Child::Done(None)
        };
        *cell.borrow_mut() = child;
    }
}

fn tree_size(node: &Node, rng: &mut Rng) -> usize {
    force(&node.left, rng);
    let mut total = 1;
    if let Child::Done(Some(ref child)) = *node.left.borrow() {
        total += tree_size(child, rng);
    }
    force(&node.right, rng);
    if let Child::Done(Some(ref child)) = *node.right.borrow() {
        total += tree_size(child, rng);
    }
    total
}

fn main() {
    let mut rng = Rng(42);
    let root = generate(); // O(1)
    println!("Generated finite tree size: {}", tree_size(&root, &mut rng));
}
