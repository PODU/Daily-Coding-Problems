// Day 196: Most frequent subtree sum.
// Postorder DFS computing each node's subtree sum, count frequencies in a HashMap.
// Time: O(n), Space: O(n).
use std::collections::HashMap;

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn dfs(n: &Option<Box<Node>>, freq: &mut HashMap<i32, i32>) -> i32 {
    match n {
        None => 0,
        Some(node) => {
            let s = node.val + dfs(&node.left, freq) + dfs(&node.right, freq);
            *freq.entry(s).or_insert(0) += 1;
            s
        }
    }
}

fn most_frequent_subtree_sum(root: &Option<Box<Node>>) -> i32 {
    let mut freq = HashMap::new();
    dfs(root, &mut freq);
    let mut best = 0;
    let mut best_count = -1;
    for (&k, &c) in &freq {
        if c > best_count {
            best_count = c;
            best = k;
        }
    }
    best
}

fn main() {
    let root = Some(Box::new(Node {
        val: 5,
        left: Some(Box::new(Node { val: 2, left: None, right: None })),
        right: Some(Box::new(Node { val: -5, left: None, right: None })),
    }));
    println!("{}", most_frequent_subtree_sum(&root)); // 2
}
