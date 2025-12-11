// Day 734: Time-travel map; get(key,t) returns value set at the most recent time <= t.
// Approach: per key a BTreeMap time->value; get takes the last entry with time <= t.
// Time: set O(log n), get O(log n). Space: O(n).
use std::collections::{BTreeMap, HashMap};

struct TimeMap {
    store: HashMap<i32, BTreeMap<i32, i32>>,
}

impl TimeMap {
    fn new() -> Self { TimeMap { store: HashMap::new() } }
    fn set(&mut self, key: i32, value: i32, time: i32) {
        self.store.entry(key).or_default().insert(time, value);
    }
    fn get(&self, key: i32, time: i32) -> Option<i32> {
        self.store.get(&key)?.range(..=time).next_back().map(|(_, &v)| v)
    }
}

fn show(v: Option<i32>) {
    match v {
        Some(x) => println!("{}", x),
        None => println!("null"),
    }
}

fn main() {
    let mut d1 = TimeMap::new();
    d1.set(1, 1, 0);
    d1.set(1, 2, 2);
    show(d1.get(1, 1)); // 1
    show(d1.get(1, 3)); // 2
    let mut d2 = TimeMap::new();
    d2.set(1, 1, 5);
    show(d2.get(1, 0));  // null
    show(d2.get(1, 10)); // 1
    let mut d3 = TimeMap::new();
    d3.set(1, 1, 0);
    d3.set(1, 2, 0);
    show(d3.get(1, 0)); // 2
}
