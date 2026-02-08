// Time-versioned map: per key a BTreeMap<time,value>; get uses range for floor.
// set/get O(log n). Setting same key+time overwrites.
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ops::Bound::{Included, Unbounded};

struct TimeMap {
    store: HashMap<i32, BTreeMap<i32, i32>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap { store: HashMap::new() }
    }
    fn set(&mut self, key: i32, value: i32, time: i32) {
        self.store.entry(key).or_insert_with(BTreeMap::new).insert(time, value);
    }
    fn get(&self, key: i32, time: i32) -> Option<i32> {
        let m = self.store.get(&key)?;
        m.range((Unbounded, Included(time))).next_back().map(|(_, &v)| v)
    }
}

fn show(v: Option<i32>) -> String {
    match v {
        Some(x) => x.to_string(),
        None => "null".to_string(),
    }
}

fn main() {
    // README presents three logical blocks; each starts from its own map state.
    let mut d = TimeMap::new();
    d.set(1, 1, 0);
    d.set(1, 2, 2);
    println!("d.get(1, 1) -> {}", show(d.get(1, 1)));
    println!("d.get(1, 3) -> {}", show(d.get(1, 3)));

    d = TimeMap::new();
    d.set(1, 1, 5);
    println!("d.get(1, 0) -> {}", show(d.get(1, 0)));
    println!("d.get(1, 10) -> {}", show(d.get(1, 10)));

    d = TimeMap::new();
    d.set(1, 1, 0);
    d.set(1, 2, 0); // overwrite same key+time -> value 2
    println!("d.get(1, 0) -> {}", show(d.get(1, 0)));
}
