// Post-order DFS: compute each subtree sum, tally counts in a HashMap, return most frequent.
// Time O(n), Space O(n).
use std::collections::HashMap;

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn dfs(node: &Option<Box<Node>>, counts: &mut HashMap<i32, i32>) -> i32 {
    match node {
        None => 0,
        Some(n) => {
            let s = n.val + dfs(&n.left, counts) + dfs(&n.right, counts);
            *counts.entry(s).or_insert(0) += 1;
            s
        }
    }
}

fn most_frequent_subtree_sum(root: &Option<Box<Node>>) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    dfs(root, &mut counts);
    let mut best = 0;
    let mut ans = 0;
    let mut first = true;
    for (&k, &v) in &counts {
        if first || v > best || (v == best && k < ans) {
            best = v;
            ans = k;
            first = false;
        }
    }
    ans
}

fn main() {
    let root = Some(Box::new(Node {
        val: 5,
        left: Some(Box::new(Node { val: 2, left: None, right: None })),
        right: Some(Box::new(Node { val: -5, left: None, right: None })),
    }));
    println!("{}", most_frequent_subtree_sum(&root));
}
