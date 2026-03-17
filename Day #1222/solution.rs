// Post-order DFS computing subtree sums, count frequencies in a hashmap,
// return sum(s) with max frequency. O(n) time, O(n) space.
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32, left: Link, right: Link) -> Link {
        Some(Rc::new(RefCell::new(Node { val, left, right })))
    }
}

fn dfs(node: &Link, freq: &mut HashMap<i32, i32>) -> i32 {
    match node {
        None => 0,
        Some(n) => {
            let n = n.borrow();
            let s = n.val + dfs(&n.left, freq) + dfs(&n.right, freq);
            *freq.entry(s).or_insert(0) += 1;
            s
        }
    }
}

fn most_frequent_subtree_sum(root: &Link) -> Vec<i32> {
    let mut freq = HashMap::new();
    dfs(root, &mut freq);
    let best = freq.values().cloned().max().unwrap_or(0);
    let mut res: Vec<i32> = freq
        .iter()
        .filter(|(_, &c)| c == best)
        .map(|(&s, _)| s)
        .collect();
    res.sort();
    res
}

fn main() {
    let root = Node::new(5, Node::new(2, None, None), Node::new(-5, None, None));
    let res = most_frequent_subtree_sum(&root);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
