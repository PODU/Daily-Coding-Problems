// Day 1501: Most frequent subtree sum.
// Approach: post-order DFS, accumulate subtree sums in a HashMap, pick max count.
// Time: O(n), Space: O(n).
use std::collections::HashMap;

struct Node {
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn dfs(node: &Option<Box<Node>>, freq: &mut HashMap<i64, i32>) -> i64 {
    match node {
        None => 0,
        Some(n) => {
            let sum = n.val + dfs(&n.left, freq) + dfs(&n.right, freq);
            *freq.entry(sum).or_insert(0) += 1;
            sum
        }
    }
}

fn most_frequent_subtree_sum(root: &Option<Box<Node>>) -> Vec<i64> {
    let mut freq: HashMap<i64, i32> = HashMap::new();
    dfs(root, &mut freq);
    let best = freq.values().cloned().max().unwrap_or(0);
    let mut res: Vec<i64> = freq.iter().filter(|&(_, &v)| v == best).map(|(&k, _)| k).collect();
    res.sort();
    res
}

fn main() {
    let root = Some(Box::new(Node {
        val: 5,
        left: Some(Box::new(Node { val: 2, left: None, right: None })),
        right: Some(Box::new(Node { val: -5, left: None, right: None })),
    }));
    let res = most_frequent_subtree_sum(&root);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
