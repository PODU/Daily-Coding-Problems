// Most frequent subtree sum: post-order DFS computes each subtree sum, count in a map.
// Time: O(n), Space: O(n).
use std::collections::HashMap;

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

fn dfs(n: &Link, count: &mut HashMap<i32, i32>) -> i32 {
    match n {
        None => 0,
        Some(b) => {
            let s = b.val + dfs(&b.left, count) + dfs(&b.right, count);
            *count.entry(s).or_insert(0) += 1;
            s
        }
    }
}

fn most_frequent_subtree_sum(root: &Link) -> i32 {
    let mut count = HashMap::new();
    dfs(root, &mut count);
    let mut best = 0;
    let mut best_count = -1;
    for (&sum, &c) in &count {
        if c > best_count {
            best_count = c;
            best = sum;
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
