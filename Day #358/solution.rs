// All O(1) structure: doubly-linked list of count-buckets (increasing) via index arena, each holds an ordered key set.
// plus/minus move key to adjacent bucket. get_max=last bucket, get_min=first bucket. All O(1).
use std::collections::HashMap;

struct Bucket {
    count: i64,
    keys: Vec<String>, // insertion-ordered representatives
    prev: usize,
    next: usize,
    alive: bool,
}

struct AllOne {
    buckets: Vec<Bucket>,
    head: usize,
    tail: usize,
    key_bucket: HashMap<String, usize>,
}

impl AllOne {
    fn new() -> Self {
        let mut buckets = Vec::new();
        buckets.push(Bucket { count: i64::MIN, keys: vec![], prev: usize::MAX, next: 1, alive: true }); // head=0
        buckets.push(Bucket { count: i64::MAX, keys: vec![], prev: 0, next: usize::MAX, alive: true }); // tail=1
        AllOne { buckets, head: 0, tail: 1, key_bucket: HashMap::new() }
    }

    fn insert_after(&mut self, b: usize, count: i64) -> usize {
        let nxt = self.buckets[b].next;
        let id = self.buckets.len();
        self.buckets.push(Bucket { count, keys: vec![], prev: b, next: nxt, alive: true });
        self.buckets[b].next = id;
        self.buckets[nxt].prev = id;
        id
    }

    fn unlink(&mut self, b: usize) {
        let (p, n) = (self.buckets[b].prev, self.buckets[b].next);
        self.buckets[p].next = n;
        self.buckets[n].prev = p;
        self.buckets[b].alive = false;
    }

    fn add_key(&mut self, b: usize, key: &str) {
        self.buckets[b].keys.push(key.to_string());
        self.key_bucket.insert(key.to_string(), b);
    }

    fn remove_key(&mut self, b: usize, key: &str) {
        if let Some(pos) = self.buckets[b].keys.iter().position(|k| k == key) {
            self.buckets[b].keys.remove(pos);
        }
    }

    fn plus(&mut self, key: &str) {
        if let Some(&cur) = self.key_bucket.get(key) {
            let nxt = self.buckets[cur].next;
            let nb = if nxt == self.tail || self.buckets[nxt].count != self.buckets[cur].count + 1 {
                self.insert_after(cur, self.buckets[cur].count + 1)
            } else { nxt };
            self.add_key(nb, key);
            self.remove_key(cur, key);
            if self.buckets[cur].keys.is_empty() { self.unlink(cur); }
        } else {
            let first = self.buckets[self.head].next;
            let nb = if first == self.tail || self.buckets[first].count != 1 {
                self.insert_after(self.head, 1)
            } else { first };
            self.add_key(nb, key);
        }
    }

    fn minus(&mut self, key: &str) {
        let cur = match self.key_bucket.get(key) { Some(&c) => c, None => return };
        if self.buckets[cur].count == 1 {
            self.remove_key(cur, key);
            self.key_bucket.remove(key);
            if self.buckets[cur].keys.is_empty() { self.unlink(cur); }
            return;
        }
        let prv = self.buckets[cur].prev;
        let nb = if prv == self.head || self.buckets[prv].count != self.buckets[cur].count - 1 {
            self.insert_after(prv, self.buckets[cur].count - 1)
        } else { prv };
        self.add_key(nb, key);
        self.remove_key(cur, key);
        if self.buckets[cur].keys.is_empty() { self.unlink(cur); }
    }

    fn get_max(&self) -> String {
        let last = self.buckets[self.tail].prev;
        if last == self.head { return String::new(); }
        self.buckets[last].keys.iter().min().cloned().unwrap_or_default()
    }

    fn get_min(&self) -> String {
        let first = self.buckets[self.head].next;
        if first == self.tail { return String::new(); }
        self.buckets[first].keys.iter().min().cloned().unwrap_or_default()
    }
}

fn main() {
    let mut a = AllOne::new();
    a.plus("a"); a.plus("b"); a.plus("b");
    a.plus("c"); a.plus("c"); a.plus("c");
    println!("max={}", a.get_max());
    println!("min={}", a.get_min());
    a.minus("c"); a.minus("c");
    println!("max={}", a.get_max());
}
