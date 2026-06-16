// Day 1676: PrefixMapSum via prefix-sum map + delta on overwrite.
// insert/sum both O(key length). Space O(total chars).
use std::collections::HashMap;

struct PrefixMapSum {
    total: HashMap<String, i64>,
    vals: HashMap<String, i64>,
}

impl PrefixMapSum {
    fn new() -> Self {
        PrefixMapSum { total: HashMap::new(), vals: HashMap::new() }
    }
    fn insert(&mut self, key: &str, value: i64) {
        let delta = value - *self.vals.get(key).unwrap_or(&0);
        self.vals.insert(key.to_string(), value);
        let mut prefix = String::new();
        for ch in key.chars() {
            prefix.push(ch);
            *self.total.entry(prefix.clone()).or_insert(0) += delta;
        }
    }
    fn sum(&self, prefix: &str) -> i64 {
        *self.total.get(prefix).unwrap_or(&0)
    }
}

fn main() {
    let mut m = PrefixMapSum::new();
    m.insert("columnar", 3);
    println!("{}", m.sum("col")); // 3
    m.insert("column", 2);
    println!("{}", m.sum("col")); // 5
}
