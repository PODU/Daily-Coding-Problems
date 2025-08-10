// Day 97: Time-versioned map. Each key keeps a BTreeMap time->value; get takes
// the last range entry with time <= query. set/get O(log n).
use std::collections::BTreeMap;
use std::collections::HashMap;

struct TimeMap {
    store: HashMap<i32, BTreeMap<i32, i32>>,
}

impl TimeMap {
    fn new() -> Self { TimeMap { store: HashMap::new() } }

    fn set(&mut self, key: i32, value: i32, time: i32) {
        self.store.entry(key).or_default().insert(time, value);
    }

    fn get(&self, key: i32, time: i32) -> Option<i32> {
        self.store
            .get(&key)?
            .range(..=time)
            .next_back()
            .map(|(_, &v)| v)
    }
}

fn show(v: Option<i32>) {
    match v {
        Some(x) => println!("{}", x),
        None => println!("null"),
    }
}

fn main() {
    // The README's three blocks are independent scenarios (fresh maps).
    let mut a = TimeMap::new();
    a.set(1, 1, 0); a.set(1, 2, 2);
    show(a.get(1, 1));  // 1
    show(a.get(1, 3));  // 2

    let mut b = TimeMap::new();
    b.set(1, 1, 5);
    show(b.get(1, 0));  // null
    show(b.get(1, 10)); // 1

    let mut c = TimeMap::new();
    c.set(1, 1, 0); c.set(1, 2, 0);
    show(c.get(1, 0)); // 2
}
