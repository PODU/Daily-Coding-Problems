// Day 1273: PrefixMapSum - insert(key,value) and sum(prefix).
// Trie storing accumulated sums; insert applies the delta vs the old value.
// insert/sum are O(key length).
use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    sum: i64,
    next: HashMap<char, TrieNode>,
}

#[derive(Default)]
struct PrefixMapSum {
    root: TrieNode,
    vals: HashMap<String, i64>,
}

impl PrefixMapSum {
    fn insert(&mut self, key: &str, value: i64) {
        let delta = value - *self.vals.get(key).unwrap_or(&0);
        self.vals.insert(key.to_string(), value);
        let mut node = &mut self.root;
        for c in key.chars() {
            node = node.next.entry(c).or_default();
            node.sum += delta;
        }
    }

    fn sum(&self, prefix: &str) -> i64 {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.next.get(&c) {
                Some(n) => node = n,
                None => return 0,
            }
        }
        node.sum
    }
}

fn main() {
    let mut mapsum = PrefixMapSum::default();
    mapsum.insert("columnar", 3);
    println!("{}", mapsum.sum("col")); // 3
    mapsum.insert("column", 2);
    println!("{}", mapsum.sum("col")); // 5
}
