// Day 426: Level of binary tree with minimum sum (levels 0-indexed; root = level 0).
// BFS level-order summing each level, track minimum. Time O(n), Space O(width).
use std::collections::VecDeque;

struct Node {
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(v: i64) -> Box<Node> {
        Box::new(Node { val: v, left: None, right: None })
    }
}

fn main() {
    let mut root = Node::new(1);
    root.left = Some(Node::new(2));
    root.right = Some(Node::new(3));

    let mut q: VecDeque<&Node> = VecDeque::new();
    q.push_back(&root);
    let mut level = 0i32;
    let mut best_level = 0i32;
    let mut best = i64::MAX;
    while !q.is_empty() {
        let sz = q.len();
        let mut s = 0i64;
        for _ in 0..sz {
            let n = q.pop_front().unwrap();
            s += n.val;
            if let Some(ref l) = n.left {
                q.push_back(l);
            }
            if let Some(ref r) = n.right {
                q.push_back(r);
            }
        }
        if s < best {
            best = s;
            best_level = level;
        }
        level += 1;
    }
    println!("Level with minimum sum: {} (sum = {})", best_level, best);
}
