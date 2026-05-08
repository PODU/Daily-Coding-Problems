// Day 1489: Time-indexed map. Per key, keep entries sorted by time; get does
// binary search for the most recent time <= query. set O(log n), get O(log n).
use std::collections::HashMap;

struct TimeMap {
    store: HashMap<i32, Vec<(i32, i32)>>, // key -> sorted (time, value)
}

impl TimeMap {
    fn new() -> Self {
        TimeMap { store: HashMap::new() }
    }

    fn set(&mut self, key: i32, value: i32, time: i32) {
        let v = self.store.entry(key).or_default();
        match v.binary_search_by(|e| e.0.cmp(&time)) {
            Ok(i) => v[i].1 = value,              // overwrite same time
            Err(i) => v.insert(i, (time, value)), // keep sorted
        }
    }

    fn get(&self, key: i32, time: i32) -> Option<i32> {
        let v = self.store.get(&key)?;
        // partition_point: count of entries with time <= query
        let idx = v.partition_point(|e| e.0 <= time);
        if idx == 0 { None } else { Some(v[idx - 1].1) }
    }
}

fn show(d: &TimeMap, key: i32, time: i32) {
    match d.get(key, time) {
        Some(val) => println!("get({},{}) = {}", key, time, val),
        None => println!("get({},{}) = null", key, time),
    }
}

fn main() {
    let mut d1 = TimeMap::new();
    d1.set(1, 1, 0);
    d1.set(1, 2, 2);
    show(&d1, 1, 1); // 1
    show(&d1, 1, 3); // 2

    let mut d2 = TimeMap::new();
    d2.set(1, 1, 5);
    show(&d2, 1, 0);  // null
    show(&d2, 1, 10); // 1

    let mut d3 = TimeMap::new();
    d3.set(1, 1, 0);
    d3.set(1, 2, 0);
    show(&d3, 1, 0); // 2
}
