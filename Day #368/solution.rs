// Day 368: KV store with update/get/max_key(val).
// kv maps key->value; by_val maps value->BTreeSet of keys, so max_key is the
// set's last element. update O(log n), get O(1) avg, max_key O(log n).
use std::collections::{BTreeSet, HashMap};

struct KVStore {
    kv: HashMap<i64, i64>,
    by_val: HashMap<i64, BTreeSet<i64>>,
}

impl KVStore {
    fn new() -> Self {
        KVStore { kv: HashMap::new(), by_val: HashMap::new() }
    }

    fn update(&mut self, key: i64, val: i64) {
        if let Some(&old) = self.kv.get(&key) {
            if let Some(set) = self.by_val.get_mut(&old) {
                set.remove(&key);
                if set.is_empty() {
                    self.by_val.remove(&old);
                }
            }
        }
        self.kv.insert(key, val);
        self.by_val.entry(val).or_default().insert(key);
    }

    fn get(&self, key: i64) -> Option<i64> {
        self.kv.get(&key).copied()
    }

    fn max_key(&self, val: i64) -> Option<i64> {
        self.by_val.get(&val).and_then(|s| s.iter().next_back().copied())
    }
}

fn main() {
    let mut kv = KVStore::new();
    kv.update(1, 1);
    kv.update(2, 1);
    println!("{}", kv.max_key(1).unwrap()); // 2
    let _ = kv.get(1);
}
