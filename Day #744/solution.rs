// O(1) LFU cache. Intrusive doubly linked list per frequency (Vec-backed nodes);
// front = most recent. Eviction: least-frequent bucket, then its tail (least recent).
// Time: O(1) per get/set, Space: O(capacity).
use std::collections::HashMap;

struct Node {
    key: i32,
    value: i32,
    freq: u64,
    prev: Option<usize>,
    next: Option<usize>,
}

#[derive(Clone, Copy)]
struct FreqList {
    head: Option<usize>,
    tail: Option<usize>,
}

struct LFUCache {
    cap: usize,
    min_freq: u64,
    nodes: Vec<Node>,
    key_to_idx: HashMap<i32, usize>,
    buckets: HashMap<u64, FreqList>,
}

impl LFUCache {
    fn new(cap: usize) -> Self {
        LFUCache {
            cap,
            min_freq: 0,
            nodes: Vec::new(),
            key_to_idx: HashMap::new(),
            buckets: HashMap::new(),
        }
    }

    fn unlink(&mut self, idx: usize) {
        let f = self.nodes[idx].freq;
        let p = self.nodes[idx].prev;
        let n = self.nodes[idx].next;
        match p {
            Some(pi) => self.nodes[pi].next = n,
            None => {
                if let Some(b) = self.buckets.get_mut(&f) {
                    b.head = n;
                }
            }
        }
        match n {
            Some(ni) => self.nodes[ni].prev = p,
            None => {
                if let Some(b) = self.buckets.get_mut(&f) {
                    b.tail = p;
                }
            }
        }
        if let Some(b) = self.buckets.get(&f) {
            if b.head.is_none() {
                self.buckets.remove(&f);
            }
        }
    }

    fn push_front(&mut self, idx: usize, f: u64) {
        self.nodes[idx].freq = f;
        let old_head = self.buckets.get(&f).and_then(|b| b.head);
        self.nodes[idx].prev = None;
        self.nodes[idx].next = old_head;
        if let Some(h) = old_head {
            self.nodes[h].prev = Some(idx);
        }
        let entry = self.buckets.entry(f).or_insert(FreqList { head: None, tail: None });
        entry.head = Some(idx);
        if entry.tail.is_none() {
            entry.tail = Some(idx);
        }
    }

    fn bump(&mut self, key: i32) {
        let idx = self.key_to_idx[&key];
        let f = self.nodes[idx].freq;
        self.unlink(idx);
        if !self.buckets.contains_key(&f) && self.min_freq == f {
            self.min_freq += 1;
        }
        self.push_front(idx, f + 1);
    }

    fn get(&mut self, key: i32) -> Option<i32> {
        if !self.key_to_idx.contains_key(&key) {
            return None;
        }
        self.bump(key);
        Some(self.nodes[self.key_to_idx[&key]].value)
    }

    fn set(&mut self, key: i32, value: i32) {
        if self.cap == 0 {
            return;
        }
        if let Some(&idx) = self.key_to_idx.get(&key) {
            self.nodes[idx].value = value;
            self.bump(key);
            return;
        }
        if self.key_to_idx.len() >= self.cap {
            let tail = self.buckets.get(&self.min_freq).unwrap().tail.unwrap();
            let ek = self.nodes[tail].key;
            self.unlink(tail);
            self.key_to_idx.remove(&ek);
        }
        let idx = self.nodes.len();
        self.nodes.push(Node { key, value, freq: 1, prev: None, next: None });
        self.key_to_idx.insert(key, idx);
        self.push_front(idx, 1);
        self.min_freq = 1;
    }
}

fn print_get(c: &mut LFUCache, key: i32) {
    match c.get(key) {
        Some(v) => println!("{}", v),
        None => println!("null"),
    }
}

fn main() {
    let mut c = LFUCache::new(2);
    c.set(1, 1);
    c.set(2, 2);
    print_get(&mut c, 1); // 1
    c.set(3, 3);          // evicts key 2
    print_get(&mut c, 2); // null
    print_get(&mut c, 3); // 3
    c.set(4, 4);          // evicts key 1
    print_get(&mut c, 1); // null
    print_get(&mut c, 3); // 3
    print_get(&mut c, 4); // 4
}
