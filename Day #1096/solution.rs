// Day 1096: LFU Cache with O(1) get/set (amortized).
// Approach: key->(val,freq) maps + freq->VecDeque(LRU order) + min_freq pointer.
// Time: O(1) per op. Space: O(n).
use std::collections::{HashMap, VecDeque};

struct LFUCache {
    cap: usize,
    min_freq: i32,
    vals: HashMap<i32, i32>,
    freqs: HashMap<i32, i32>,
    lists: HashMap<i32, VecDeque<i32>>, // freq -> keys, front = LRU, back = MRU
}

impl LFUCache {
    fn new(n: usize) -> Self {
        LFUCache { cap: n, min_freq: 0, vals: HashMap::new(), freqs: HashMap::new(), lists: HashMap::new() }
    }

    fn touch(&mut self, key: i32) {
        let f = self.freqs[&key];
        if let Some(dq) = self.lists.get_mut(&f) {
            if let Some(pos) = dq.iter().position(|&k| k == key) {
                dq.remove(pos);
            }
            if dq.is_empty() {
                self.lists.remove(&f);
                if self.min_freq == f { self.min_freq += 1; }
            }
        }
        let nf = f + 1;
        self.freqs.insert(key, nf);
        self.lists.entry(nf).or_default().push_back(key);
    }

    fn get(&mut self, key: i32) -> Option<i32> {
        if !self.vals.contains_key(&key) { return None; }
        self.touch(key);
        Some(self.vals[&key])
    }

    fn set(&mut self, key: i32, value: i32) {
        if self.cap == 0 { return; }
        if self.vals.contains_key(&key) {
            self.vals.insert(key, value);
            self.touch(key);
            return;
        }
        if self.vals.len() >= self.cap {
            let evict = {
                let dq = self.lists.get_mut(&self.min_freq).unwrap();
                dq.pop_front().unwrap()
            };
            if self.lists[&self.min_freq].is_empty() { self.lists.remove(&self.min_freq); }
            self.vals.remove(&evict);
            self.freqs.remove(&evict);
        }
        self.vals.insert(key, value);
        self.freqs.insert(key, 1);
        self.lists.entry(1).or_default().push_back(key);
        self.min_freq = 1;
    }
}

fn main() {
    let mut c = LFUCache::new(2);
    c.set(1, 1);
    c.set(2, 2);
    println!("{}", c.get(1).unwrap()); // 1
    c.set(3, 3);                       // evicts key 2
    match c.get(2) { Some(v) => println!("{}", v), None => println!("null") }; // null
    println!("{}", c.get(3).unwrap()); // 3
}
