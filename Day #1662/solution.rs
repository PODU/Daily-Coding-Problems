// O(1) LFU cache: key->node map, freq->ordered list(by recency), minFreq pointer.
// get/set O(1) amortized; Space O(capacity). Evict least-freq, tie -> least-recently-used.
use std::collections::HashMap;
use std::collections::VecDeque;

struct LFU {
    cap: usize,
    min_freq: i64,
    vals: HashMap<i32, i32>,
    freqs: HashMap<i32, i64>,
    lists: HashMap<i64, VecDeque<i32>>, // freq -> keys (front=LRU, back=MRU)
}
impl LFU {
    fn new(cap: usize) -> Self {
        LFU { cap, min_freq: 0, vals: HashMap::new(), freqs: HashMap::new(), lists: HashMap::new() }
    }
    fn touch(&mut self, key: i32) {
        let f = self.freqs[&key];
        if let Some(dq) = self.lists.get_mut(&f) {
            if let Some(pos) = dq.iter().position(|&k| k == key) { dq.remove(pos); }
            if dq.is_empty() { self.lists.remove(&f); if self.min_freq == f { self.min_freq += 1; } }
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
    fn set(&mut self, key: i32, val: i32) {
        if self.cap == 0 { return; }
        if self.vals.contains_key(&key) {
            self.vals.insert(key, val);
            self.touch(key);
            return;
        }
        if self.vals.len() >= self.cap {
            let dq = self.lists.get_mut(&self.min_freq).unwrap();
            let lru = dq.pop_front().unwrap();
            if dq.is_empty() { self.lists.remove(&self.min_freq); }
            self.vals.remove(&lru);
            self.freqs.remove(&lru);
        }
        self.vals.insert(key, val);
        self.freqs.insert(key, 1);
        self.lists.entry(1).or_default().push_back(key);
        self.min_freq = 1;
    }
}
fn show(v: Option<i32>) {
    match v {
        Some(x) => println!("{}", x),
        None => println!("null"),
    }
}
fn main() {
    let mut c = LFU::new(2);
    c.set(1, 1);
    c.set(2, 2);
    show(c.get(1));
    c.set(3, 3);
    show(c.get(2));
    show(c.get(3));
    c.set(4, 4);
    show(c.get(1));
    show(c.get(3));
    show(c.get(4));
}
