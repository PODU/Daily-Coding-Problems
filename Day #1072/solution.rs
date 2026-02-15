// BFS level-order traversal, track sum per level; return 1-indexed level with min sum. O(n) time, O(n) space.
use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i64, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node { val, left, right })
    }
    fn leaf(val: i64) -> Box<Node> {
        Box::new(Node { val, left: None, right: None })
    }
}

fn min_sum_level(root: &Node) -> (usize, i64) {
    let mut q: VecDeque<*const Node> = VecDeque::new();
    q.push_back(root as *const Node);
    let mut level = 1usize;
    let mut min_level = 1usize;
    let mut min_sum = i64::MAX;
    while !q.is_empty() {
        let sz = q.len();
        let mut sum = 0i64;
        for _ in 0..sz {
            let cur = unsafe { &*q.pop_front().unwrap() };
            sum += cur.val;
            if let Some(l) = &cur.left  { q.push_back(l.as_ref() as *const Node); }
            if let Some(r) = &cur.right { q.push_back(r.as_ref() as *const Node); }
        }
        if sum < min_sum { min_sum = sum; min_level = level; }
        level += 1;
    }
    (min_level, min_sum)
}

fn main() {
    // Tree 1: 1 -> (2->(4,5), 3->(_,6))
    let r1 = Node::new(1,
        Some(Node::new(2, Some(Node::leaf(4)), Some(Node::leaf(5)))),
        Some(Node::new(3, None, Some(Node::leaf(6)))));
    let (lvl, sm) = min_sum_level(&r1);
    println!("Level with min sum: {} (sum={})", lvl, sm);

    // Tree 2: 10 -> (2, 3)
    let r2 = Node::new(10, Some(Node::leaf(2)), Some(Node::leaf(3)));
    let (lvl, sm) = min_sum_level(&r2);
    println!("Level with min sum: {} (sum={})", lvl, sm);
}
