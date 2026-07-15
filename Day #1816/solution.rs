// Time-versioned map: per key keep times sorted; get returns value at most-recent time <= t.
// set amortized O(log m), get O(log m) via binary search. Space: O(total entries).
use std::collections::HashMap;

struct TimeMap {
    data: HashMap<i32, Vec<(i32, i32)>>, // key -> sorted (time, value)
}

impl TimeMap {
    fn new() -> Self {
        TimeMap { data: HashMap::new() }
    }
    fn set(&mut self, key: i32, value: i32, time: i32) {
        let arr = self.data.entry(key).or_default();
        match arr.binary_search_by(|e| e.0.cmp(&time)) {
            Ok(i) => arr[i].1 = value,
            Err(i) => arr.insert(i, (time, value)),
        }
    }
    fn get(&self, key: i32, time: i32) -> Option<i32> {
        let arr = self.data.get(&key)?;
        // partition_point: first index with time_i > query
        let i = arr.partition_point(|e| e.0 <= time);
        if i == 0 {
            None
        } else {
            Some(arr[i - 1].1)
        }
    }
}

fn show(d: &TimeMap, key: i32, time: i32) {
    match d.get(key, time) {
        Some(v) => println!("get({}, {}) = {}", key, time, v),
        None => println!("get({}, {}) = null", key, time),
    }
}

fn main() {
    let mut d = TimeMap::new(); d.set(1, 1, 0); d.set(1, 2, 2); show(&d, 1, 1); show(&d, 1, 3); // 1, 2
    let mut d = TimeMap::new(); d.set(1, 1, 5); show(&d, 1, 0); show(&d, 1, 10);                 // null, 1
    let mut d = TimeMap::new(); d.set(1, 1, 0); d.set(1, 2, 0); show(&d, 1, 0);                  // 2
}
