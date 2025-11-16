// Day 611: All O(1) structure (plus / minus / get_max / get_min).
// Approach: doubly-linked list of value-buckets (set of keys) + key->bucket map (indices). All ops O(1).
use std::collections::{HashMap, HashSet};

// Buckets stored in a slab (Vec) with a free list; links are indices. 0=head, 1=tail sentinels.
struct Bucket {
    value: i64,
    keys: HashSet<String>,
    prev: usize,
    next: usize,
}

struct AllOne {
    buckets: Vec<Bucket>,
    free: Vec<usize>,
    head: usize,
    tail: usize,
    key_to: HashMap<String, usize>,
}

impl AllOne {
    fn new() -> Self {
        let head = Bucket { value: 0, keys: HashSet::new(), prev: 0, next: 1 };
        let tail = Bucket { value: 0, keys: HashSet::new(), prev: 0, next: 1 };
        AllOne {
            buckets: vec![head, tail],
            free: Vec::new(),
            head: 0,
            tail: 1,
            key_to: HashMap::new(),
        }
    }

    fn alloc(&mut self, value: i64) -> usize {
        if let Some(i) = self.free.pop() {
            self.buckets[i].value = value;
            self.buckets[i].keys.clear();
            i
        } else {
            self.buckets.push(Bucket { value, keys: HashSet::new(), prev: 0, next: 0 });
            self.buckets.len() - 1
        }
    }

    fn insert_after(&mut self, node: usize, value: i64) -> usize {
        let nxt = self.buckets[node].next;
        let b = self.alloc(value);
        self.buckets[b].prev = node;
        self.buckets[b].next = nxt;
        self.buckets[node].next = b;
        self.buckets[nxt].prev = b;
        b
    }

    fn remove(&mut self, node: usize) {
        let (p, n) = (self.buckets[node].prev, self.buckets[node].next);
        self.buckets[p].next = n;
        self.buckets[n].prev = p;
        self.free.push(node);
    }

    fn plus(&mut self, key: &str) {
        if let Some(&cur) = self.key_to.get(key) {
            let cv = self.buckets[cur].value;
            let mut nxt = self.buckets[cur].next;
            if nxt == self.tail || self.buckets[nxt].value != cv + 1 {
                nxt = self.insert_after(cur, cv + 1);
            }
            self.buckets[nxt].keys.insert(key.to_string());
            self.key_to.insert(key.to_string(), nxt);
            self.buckets[cur].keys.remove(key);
            if self.buckets[cur].keys.is_empty() {
                self.remove(cur);
            }
        } else {
            let mut first = self.buckets[self.head].next;
            if first == self.tail || self.buckets[first].value != 1 {
                first = self.insert_after(self.head, 1);
            }
            self.buckets[first].keys.insert(key.to_string());
            self.key_to.insert(key.to_string(), first);
        }
    }

    fn minus(&mut self, key: &str) {
        let cur = match self.key_to.get(key) {
            Some(&c) => c,
            None => return,
        };
        let cv = self.buckets[cur].value;
        if cv == 1 {
            self.key_to.remove(key);
        } else {
            let prev = self.buckets[cur].prev;
            let mut prv = prev;
            if prv == self.head || self.buckets[prv].value != cv - 1 {
                prv = self.insert_after(prev, cv - 1);
            }
            self.buckets[prv].keys.insert(key.to_string());
            self.key_to.insert(key.to_string(), prv);
        }
        self.buckets[cur].keys.remove(key);
        if self.buckets[cur].keys.is_empty() {
            self.remove(cur);
        }
    }

    fn get_max(&self) -> String {
        let p = self.buckets[self.tail].prev;
        if p == self.head { String::new() } else { self.buckets[p].keys.iter().min().unwrap().clone() }
    }

    fn get_min(&self) -> String {
        let n = self.buckets[self.head].next;
        if n == self.tail { String::new() } else { self.buckets[n].keys.iter().min().unwrap().clone() }
    }
}

fn main() {
    let mut a = AllOne::new();
    a.plus("a");
    a.plus("b");
    a.plus("a"); // a=2, b=1
    println!("{}", a.get_max()); // a
    println!("{}", a.get_min()); // b
}
