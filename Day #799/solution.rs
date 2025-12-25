// Day 799: PrefixMapSum - trie where each node stores sum of values below it.
// insert overwrites via delta (new-old) propagated along the path.
// insert O(L), sum O(L). Space O(total chars).
use std::collections::HashMap;

#[derive(Default)]
struct Node {
    total: i64,
    child: HashMap<char, Node>,
}

#[derive(Default)]
struct PrefixMapSum {
    root: Node,
    vals: HashMap<String, i64>,
}

impl PrefixMapSum {
    fn insert(&mut self, key: &str, value: i64) {
        let delta = value - *self.vals.get(key).unwrap_or(&0);
        self.vals.insert(key.to_string(), value);
        let mut cur = &mut self.root;
        cur.total += delta;
        for c in key.chars() {
            cur = cur.child.entry(c).or_default();
            cur.total += delta;
        }
    }

    fn sum(&self, prefix: &str) -> i64 {
        let mut cur = &self.root;
        for c in prefix.chars() {
            match cur.child.get(&c) {
                Some(n) => cur = n,
                None => return 0,
            }
        }
        cur.total
    }
}

fn main() {
    let mut m = PrefixMapSum::default();
    m.insert("columnar", 3);
    println!("{}", m.sum("col")); // 3
    m.insert("column", 2);
    println!("{}", m.sum("col")); // 5
}
