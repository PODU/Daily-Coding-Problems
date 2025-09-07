// PrefixMapSum: Trie where each node stores the running sum of values passing through it.
// On overwrite, propagate delta = new - old. insert/sum both O(key length).
use std::collections::HashMap;

#[derive(Default)]
struct Node {
    next: HashMap<char, Node>,
    sum: i32,
}

#[derive(Default)]
struct PrefixMapSum {
    root: Node,
    vals: HashMap<String, i32>,
}

impl PrefixMapSum {
    fn insert(&mut self, key: &str, value: i32) {
        let delta = value - *self.vals.get(key).unwrap_or(&0);
        self.vals.insert(key.to_string(), value);
        let mut n = &mut self.root;
        for c in key.chars() {
            n = n.next.entry(c).or_default();
            n.sum += delta;
        }
    }

    fn sum(&self, prefix: &str) -> i32 {
        let mut n = &self.root;
        for c in prefix.chars() {
            match n.next.get(&c) {
                Some(child) => n = child,
                None => return 0,
            }
        }
        n.sum
    }
}

fn main() {
    let mut mapsum = PrefixMapSum::default();
    mapsum.insert("columnar", 3);
    println!("{}", mapsum.sum("col")); // 3
    mapsum.insert("column", 2);
    println!("{}", mapsum.sum("col")); // 5
}
