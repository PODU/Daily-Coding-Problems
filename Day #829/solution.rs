// Day 829: O(1) data structure with plus/minus/get_max/get_min.
// Arena-based doubly-linked list of count-buckets (each a set of keys) + key->bucket index map.
// All operations O(1) time; O(K) space for K distinct keys.
use std::collections::{HashMap, HashSet};

struct Bucket {
    count: i64,
    keys: HashSet<String>,
    prev: usize,
    next: usize,
    alive: bool,
}

struct AllOne {
    arena: Vec<Bucket>,
    free: Vec<usize>,
    head: usize,
    tail: usize,
    key_bucket: HashMap<String, usize>,
}

impl AllOne {
    fn new() -> Self {
        let mut arena = Vec::new();
        // head sentinel = index 0, tail sentinel = index 1
        arena.push(Bucket { count: i64::MIN, keys: HashSet::new(), prev: usize::MAX, next: 1, alive: true });
        arena.push(Bucket { count: i64::MAX, keys: HashSet::new(), prev: 0, next: usize::MAX, alive: true });
        AllOne { arena, free: Vec::new(), head: 0, tail: 1, key_bucket: HashMap::new() }
    }

    fn alloc(&mut self, count: i64) -> usize {
        if let Some(i) = self.free.pop() {
            self.arena[i] = Bucket { count, keys: HashSet::new(), prev: usize::MAX, next: usize::MAX, alive: true };
            i
        } else {
            self.arena.push(Bucket { count, keys: HashSet::new(), prev: usize::MAX, next: usize::MAX, alive: true });
            self.arena.len() - 1
        }
    }

    // Insert a new bucket with `count` immediately after `node`; returns its index.
    fn insert_after(&mut self, node: usize, count: i64) -> usize {
        let b = self.alloc(count);
        let nxt = self.arena[node].next;
        self.arena[b].prev = node;
        self.arena[b].next = nxt;
        self.arena[nxt].prev = b;
        self.arena[node].next = b;
        b
    }

    fn remove(&mut self, node: usize) {
        let p = self.arena[node].prev;
        let n = self.arena[node].next;
        self.arena[p].next = n;
        self.arena[n].prev = p;
        self.arena[node].alive = false;
        self.free.push(node);
    }

    fn plus(&mut self, key: &str) {
        if let Some(&cur) = self.key_bucket.get(key) {
            let new_count = self.arena[cur].count + 1;
            let mut nxt = self.arena[cur].next;
            if self.arena[nxt].count != new_count {
                nxt = self.insert_after(cur, new_count);
            }
            self.arena[nxt].keys.insert(key.to_string());
            self.key_bucket.insert(key.to_string(), nxt);
            self.arena[cur].keys.remove(key);
            if self.arena[cur].keys.is_empty() {
                self.remove(cur);
            }
        } else {
            let mut first = self.arena[self.head].next;
            if self.arena[first].count != 1 {
                first = self.insert_after(self.head, 1);
            }
            self.arena[first].keys.insert(key.to_string());
            self.key_bucket.insert(key.to_string(), first);
        }
    }

    fn minus(&mut self, key: &str) {
        let cur = match self.key_bucket.get(key) {
            Some(&c) => c,
            None => return,
        };
        if self.arena[cur].count == 1 {
            self.arena[cur].keys.remove(key);
            self.key_bucket.remove(key);
            if self.arena[cur].keys.is_empty() {
                self.remove(cur);
            }
            return;
        }
        let new_count = self.arena[cur].count - 1;
        let prev = self.arena[cur].prev;
        let prv = if self.arena[prev].count != new_count {
            self.insert_after(prev, new_count)
        } else {
            prev
        };
        self.arena[prv].keys.insert(key.to_string());
        self.key_bucket.insert(key.to_string(), prv);
        self.arena[cur].keys.remove(key);
        if self.arena[cur].keys.is_empty() {
            self.remove(cur);
        }
    }

    fn get_max(&self) -> String {
        let last = self.arena[self.tail].prev;
        if last == self.head {
            return String::new();
        }
        self.arena[last].keys.iter().min().cloned().unwrap_or_default()
    }

    fn get_min(&self) -> String {
        let first = self.arena[self.head].next;
        if first == self.tail {
            return String::new();
        }
        self.arena[first].keys.iter().min().cloned().unwrap_or_default()
    }
}

fn main() {
    let mut ao = AllOne::new();
    ao.plus("a");
    ao.plus("b");
    ao.plus("b");
    println!("get_max: {}", ao.get_max()); // b
    println!("get_min: {}", ao.get_min()); // a
    ao.minus("b");
    ao.minus("b");
    println!("get_max: {}", ao.get_max()); // a
}
