// LFU cache: key->(value,freq), freq->ordered key list (LRU within freq via VecDeque), track minFreq. O(1) amortized.
use std::collections::HashMap;
use std::collections::VecDeque;

struct LFUCache {
    cap: usize,
    min_freq: i32,
    values: HashMap<i32, i32>,            // key -> value
    freqs: HashMap<i32, i32>,             // key -> freq
    freq_list: HashMap<i32, VecDeque<i32>>, // freq -> keys (front = LRU, back = MRU)
}

impl LFUCache {
    fn new(capacity: usize) -> Self {
        LFUCache {
            cap: capacity,
            min_freq: 0,
            values: HashMap::new(),
            freqs: HashMap::new(),
            freq_list: HashMap::new(),
        }
    }

    fn touch(&mut self, key: i32) {
        let f = *self.freqs.get(&key).unwrap();
        if let Some(dq) = self.freq_list.get_mut(&f) {
            if let Some(idx) = dq.iter().position(|&k| k == key) {
                dq.remove(idx);
            }
            if dq.is_empty() {
                self.freq_list.remove(&f);
                if self.min_freq == f {
                    self.min_freq = f + 1;
                }
            }
        }
        let nf = f + 1;
        self.freqs.insert(key, nf);
        self.freq_list.entry(nf).or_insert_with(VecDeque::new).push_back(key);
    }

    fn get(&mut self, key: i32) -> Option<i32> {
        if !self.values.contains_key(&key) {
            return None;
        }
        self.touch(key);
        Some(*self.values.get(&key).unwrap())
    }

    fn set(&mut self, key: i32, value: i32) {
        if self.cap == 0 {
            return;
        }
        if self.values.contains_key(&key) {
            self.values.insert(key, value);
            self.touch(key);
            return;
        }
        if self.values.len() >= self.cap {
            if let Some(dq) = self.freq_list.get_mut(&self.min_freq) {
                if let Some(evict) = dq.pop_front() { // LRU at min freq
                    if dq.is_empty() {
                        self.freq_list.remove(&self.min_freq);
                    }
                    self.values.remove(&evict);
                    self.freqs.remove(&evict);
                }
            }
        }
        self.values.insert(key, value);
        self.freqs.insert(key, 1);
        self.freq_list.entry(1).or_insert_with(VecDeque::new).push_back(key);
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
    let mut c = LFUCache::new(2);
    c.set(1, 1);
    c.set(2, 2);
    show(c.get(1)); // 1
    c.set(3, 3);    // evicts key 2
    show(c.get(2)); // null
    show(c.get(3)); // 3
    c.set(4, 4);    // evicts key 1
    show(c.get(1)); // null
    show(c.get(3)); // 3
    show(c.get(4)); // 4
}
