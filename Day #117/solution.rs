// Day 117: BFS level by level, track level with minimum sum. O(n) time, O(w) space.
use std::collections::VecDeque;

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node {
    fn new(val: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node { val, left, right })
    }
}

fn min_sum_level(root: &Node) -> i32 {
    let mut q: VecDeque<&Node> = VecDeque::new();
    q.push_back(root);
    let (mut level, mut best) = (0i32, 0i32);
    let mut best_sum = i64::MAX;
    while !q.is_empty() {
        let sz = q.len();
        let mut sum: i64 = 0;
        for _ in 0..sz {
            let n = q.pop_front().unwrap();
            sum += n.val as i64;
            if let Some(ref l) = n.left {
                q.push_back(l);
            }
            if let Some(ref r) = n.right {
                q.push_back(r);
            }
        }
        if sum < best_sum {
            best_sum = sum;
            best = level;
        }
        level += 1;
    }
    best
}

fn main() {
    let root = Node::new(
        10,
        Some(Node::new(2, Some(Node::new(-5, None, None)), Some(Node::new(1, None, None)))),
        Some(Node::new(3, None, None)),
    );
    println!("{}", min_sum_level(&root)); // 2
}
