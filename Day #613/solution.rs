// Day 613: PrefixMapSum - insert(key,value) and sum(prefix).
// Approach: trie where each node stores total of values passing through; insert propagates delta. Time O(|key|).
use std::collections::HashMap;

#[derive(Default)]
struct Node {
    sum: i64,
    ch: HashMap<char, Node>,
}

struct PrefixMapSum {
    root: Node,
    values: HashMap<String, i64>,
}

impl PrefixMapSum {
    fn new() -> Self {
        PrefixMapSum { root: Node::default(), values: HashMap::new() }
    }

    fn insert(&mut self, key: &str, value: i64) {
        let delta = value - *self.values.get(key).unwrap_or(&0);
        self.values.insert(key.to_string(), value);
        let mut node = &mut self.root;
        for c in key.chars() {
            node = node.ch.entry(c).or_default();
            node.sum += delta;
        }
    }

    fn sum(&self, prefix: &str) -> i64 {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.ch.get(&c) {
                Some(n) => node = n,
                None => return 0,
            }
        }
        node.sum
    }
}

fn main() {
    let mut m = PrefixMapSum::new();
    m.insert("columnar", 3);
    println!("{}", m.sum("col")); // 3
    m.insert("column", 2);
    println!("{}", m.sum("col")); // 5
}
