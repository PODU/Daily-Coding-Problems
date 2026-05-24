// Lazy binary tree: generate() returns a root in O(1) whose children are thunks
// (boxed closures) forced on demand; a seeded coin flip (<1 continue prob) makes the
// tree finite w.p.1 but unbounded. Realization is depth-capped for a deterministic demo.
use std::cell::RefCell;
use std::rc::Rc;

struct Lcg {
    s: u64,
}
impl Lcg {
    fn next(&mut self) -> u64 {
        self.s = (self.s.wrapping_mul(16807)) % 2147483647; // Park-Miller
        self.s
    }
    fn coin(&mut self) -> bool {
        self.next() % 100 < 45 // child exists prob 0.45 -> finite
    }
}

struct Node {
    #[allow(dead_code)]
    val: i32,
    left: Box<dyn Fn() -> Option<Node>>,
    right: Box<dyn Fn() -> Option<Node>>,
}

// makeNode/generate do NOT force children: O(1).
fn make_node(rng: Rc<RefCell<Lcg>>, counter: Rc<RefCell<i32>>) -> Node {
    let v = *counter.borrow();
    *counter.borrow_mut() += 1;
    let r1 = rng.clone();
    let c1 = counter.clone();
    let r2 = rng.clone();
    let c2 = counter.clone();
    Node {
        val: v,
        left: Box::new(move || {
            if r1.borrow_mut().coin() {
                Some(make_node(r1.clone(), c1.clone()))
            } else {
                None
            }
        }),
        right: Box::new(move || {
            if r2.borrow_mut().coin() {
                Some(make_node(r2.clone(), c2.clone()))
            } else {
                None
            }
        }),
    }
}

fn generate(rng: Rc<RefCell<Lcg>>, counter: Rc<RefCell<i32>>) -> Node {
    make_node(rng, counter) // O(1): one node, children unevaluated
}

fn realize(node: &Node, depth: i32, cap: i32) -> i32 {
    let mut count = 1;
    if depth < cap {
        if let Some(l) = (node.left)() {
            count += realize(&l, depth + 1, cap);
        }
        if let Some(r) = (node.right)() {
            count += realize(&r, depth + 1, cap);
        }
    }
    count
}

fn main() {
    let rng = Rc::new(RefCell::new(Lcg { s: 42 }));
    let counter = Rc::new(RefCell::new(0));
    let root = generate(rng, counter); // returns instantly
    println!("generate() returned a lazy tree root in O(1)");
    let n = realize(&root, 0, 6);
    println!("Realized tree node count: {}", n);
}
