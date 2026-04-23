// Day 1412: generate() returns a finite but arbitrarily large binary tree in O(1).
// Approach: lazy nodes — children are materialized only on access (here via a deterministic
// LCG so the demo is reproducible). generate() itself is O(1); a child appears on first touch.

static mut LCG: u64 = 42;

fn next_rand() -> i32 {
    unsafe {
        LCG = LCG.wrapping_mul(1103515245).wrapping_add(12345);
        ((LCG >> 16) & 0x7fff) as i32
    }
}

struct LazyNode {
    depth: i32,
    left_done: bool,
    right_done: bool,
    left_cache: Option<Box<LazyNode>>,
    right_cache: Option<Box<LazyNode>>,
}

impl LazyNode {
    fn new(depth: i32) -> LazyNode {
        LazyNode { depth, left_done: false, right_done: false, left_cache: None, right_cache: None }
    }
    fn spawn(&self) -> bool {
        let bound = 55 - self.depth * 7;
        bound > 0 && next_rand() % 100 < bound
    }
    fn left(&mut self) -> Option<&mut Box<LazyNode>> {
        if !self.left_done {
            self.left_done = true;
            if self.spawn() {
                self.left_cache = Some(Box::new(LazyNode::new(self.depth + 1)));
            }
        }
        self.left_cache.as_mut()
    }
    fn right(&mut self) -> Option<&mut Box<LazyNode>> {
        if !self.right_done {
            self.right_done = true;
            if self.spawn() {
                self.right_cache = Some(Box::new(LazyNode::new(self.depth + 1)));
            }
        }
        self.right_cache.as_mut()
    }
}

// generate(): O(1)
fn generate() -> LazyNode {
    LazyNode::new(0)
}

fn count_nodes(n: &mut LazyNode) -> i32 {
    let l = match n.left() {
        Some(c) => count_nodes(c),
        None => 0,
    };
    let r = match n.right() {
        Some(c) => count_nodes(c),
        None => 0,
    };
    1 + l + r
}

fn main() {
    let mut root = generate();
    println!("Tree size: {}", count_nodes(&mut root));
}
