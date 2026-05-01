// LFU cache: arena-based nodes, HashMap key->idx, HashMap freq->doubly-linked-list, track min_freq. O(1) per op.
// Time: O(1) get/set. Space: O(capacity).
use std::collections::HashMap;

struct N {
    key: i32,
    val: i32,
    freq: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

struct LFUCache {
    cap: usize,
    min_freq: i32,
    arena: Vec<N>,
    free: Vec<usize>,
    key_to_idx: HashMap<i32, usize>,
    heads: HashMap<i32, usize>, // freq -> front node idx (most recent)
    tails: HashMap<i32, usize>, // freq -> back node idx (least recent)
    sizes: HashMap<i32, usize>,
}

impl LFUCache {
    fn new(capacity: usize) -> Self {
        LFUCache {
            cap: capacity,
            min_freq: 0,
            arena: Vec::new(),
            free: Vec::new(),
            key_to_idx: HashMap::new(),
            heads: HashMap::new(),
            tails: HashMap::new(),
            sizes: HashMap::new(),
        }
    }

    fn alloc(&mut self, key: i32, val: i32, freq: i32) -> usize {
        let node = N { key, val, freq, prev: None, next: None };
        if let Some(i) = self.free.pop() {
            self.arena[i] = node;
            i
        } else {
            self.arena.push(node);
            self.arena.len() - 1
        }
    }

    fn push_front(&mut self, f: i32, idx: usize) {
        let old_head = self.heads.get(&f).copied();
        self.arena[idx].prev = None;
        self.arena[idx].next = old_head;
        if let Some(h) = old_head {
            self.arena[h].prev = Some(idx);
        } else {
            self.tails.insert(f, idx);
        }
        self.heads.insert(f, idx);
        *self.sizes.entry(f).or_insert(0) += 1;
    }

    fn unlink(&mut self, f: i32, idx: usize) {
        let prev = self.arena[idx].prev;
        let next = self.arena[idx].next;
        match prev {
            Some(p) => self.arena[p].next = next,
            None => {
                if let Some(n) = next {
                    self.heads.insert(f, n);
                } else {
                    self.heads.remove(&f);
                }
            }
        }
        match next {
            Some(n) => self.arena[n].prev = prev,
            None => {
                if let Some(p) = prev {
                    self.tails.insert(f, p);
                } else {
                    self.tails.remove(&f);
                }
            }
        }
        let s = self.sizes.entry(f).or_insert(0);
        if *s > 0 {
            *s -= 1;
        }
        if *s == 0 {
            self.sizes.remove(&f);
        }
    }

    fn touch(&mut self, idx: usize) {
        let f = self.arena[idx].freq;
        self.unlink(f, idx);
        if !self.sizes.contains_key(&f) && self.min_freq == f {
            self.min_freq += 1;
        }
        self.arena[idx].freq = f + 1;
        self.push_front(f + 1, idx);
    }

    fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(&idx) = self.key_to_idx.get(&key) {
            let v = self.arena[idx].val;
            self.touch(idx);
            Some(v)
        } else {
            None
        }
    }

    fn set(&mut self, key: i32, value: i32) {
        if self.cap == 0 {
            return;
        }
        if let Some(&idx) = self.key_to_idx.get(&key) {
            self.arena[idx].val = value;
            self.touch(idx);
            return;
        }
        if self.key_to_idx.len() >= self.cap {
            let mf = self.min_freq;
            let victim = *self.tails.get(&mf).unwrap();
            let vkey = self.arena[victim].key;
            self.unlink(mf, victim);
            self.key_to_idx.remove(&vkey);
            self.free.push(victim);
        }
        let idx = self.alloc(key, value, 1);
        self.key_to_idx.insert(key, idx);
        self.push_front(1, idx);
        self.min_freq = 1;
    }
}

fn fmt(x: Option<i32>) -> String {
    match x {
        Some(v) => v.to_string(),
        None => "null".to_string(),
    }
}

fn main() {
    let mut cache = LFUCache::new(2);
    cache.set(1, 1);
    cache.set(2, 2);
    println!("{}", fmt(cache.get(1))); // 1
    cache.set(3, 3); // evicts key 2
    println!("{}", fmt(cache.get(2))); // null
    cache.get(3); // access key 3 (raises its freq) so key 1 becomes LFU/LRU victim
    cache.set(4, 4); // evicts key 1
    println!("{}", fmt(cache.get(1))); // null
    println!("{}", fmt(cache.get(3))); // 3
    println!("{}", fmt(cache.get(4))); // 4
}
