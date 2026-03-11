// All O(1) data structure (LeetCode 432).
// Index-based doubly linked list of count-buckets (each a set of keys) + map key->bucket index. All ops O(1); space O(N).
use std::collections::{HashMap, HashSet};

struct Bucket {
    count: i64,
    keys: HashSet<String>,
    prev: usize,
    next: usize,
}

struct AllOne {
    buckets: Vec<Bucket>,
    free: Vec<usize>,
    head: usize,
    tail: usize,
    key_bucket: HashMap<String, usize>,
}

impl AllOne {
    fn new() -> Self {
        let head = Bucket { count: i64::MIN, keys: HashSet::new(), prev: 0, next: 1 };
        let tail = Bucket { count: i64::MAX, keys: HashSet::new(), prev: 0, next: 1 };
        AllOne {
            buckets: vec![head, tail],
            free: Vec::new(),
            head: 0,
            tail: 1,
            key_bucket: HashMap::new(),
        }
    }

    fn alloc(&mut self, count: i64) -> usize {
        if let Some(i) = self.free.pop() {
            self.buckets[i].count = count;
            self.buckets[i].keys.clear();
            i
        } else {
            self.buckets.push(Bucket { count, keys: HashSet::new(), prev: 0, next: 0 });
            self.buckets.len() - 1
        }
    }

    fn insert_after(&mut self, node: usize, count: i64) -> usize {
        let nxt = self.buckets[node].next;
        let b = self.alloc(count);
        self.buckets[b].prev = node;
        self.buckets[b].next = nxt;
        self.buckets[node].next = b;
        self.buckets[nxt].prev = b;
        b
    }

    fn remove(&mut self, b: usize) {
        let p = self.buckets[b].prev;
        let n = self.buckets[b].next;
        self.buckets[p].next = n;
        self.buckets[n].prev = p;
        self.free.push(b);
    }

    fn plus(&mut self, key: &str) {
        if let Some(&cur) = self.key_bucket.get(key) {
            let cur_count = self.buckets[cur].count;
            let mut nxt = self.buckets[cur].next;
            if nxt == self.tail || self.buckets[nxt].count != cur_count + 1 {
                nxt = self.insert_after(cur, cur_count + 1);
            }
            self.buckets[nxt].keys.insert(key.to_string());
            self.key_bucket.insert(key.to_string(), nxt);
            self.buckets[cur].keys.remove(key);
            if self.buckets[cur].keys.is_empty() {
                self.remove(cur);
            }
        } else {
            let mut first = self.buckets[self.head].next;
            if first == self.tail || self.buckets[first].count != 1 {
                first = self.insert_after(self.head, 1);
            }
            self.buckets[first].keys.insert(key.to_string());
            self.key_bucket.insert(key.to_string(), first);
        }
    }

    fn minus(&mut self, key: &str) {
        let cur = match self.key_bucket.get(key) {
            Some(&c) => c,
            None => return,
        };
        let cur_count = self.buckets[cur].count;
        if cur_count == 1 {
            self.key_bucket.remove(key);
        } else {
            let prev_node = self.buckets[cur].prev;
            let mut prv = prev_node;
            if prv == self.head || self.buckets[prv].count != cur_count - 1 {
                prv = self.insert_after(prev_node, cur_count - 1);
            }
            self.buckets[prv].keys.insert(key.to_string());
            self.key_bucket.insert(key.to_string(), prv);
        }
        self.buckets[cur].keys.remove(key);
        if self.buckets[cur].keys.is_empty() {
            self.remove(cur);
        }
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
    a.plus("a");
    a.plus("a");
    a.plus("a");
    a.plus("b");
    println!("get_max: {}", a.get_max());
    println!("get_min: {}", a.get_min());
    a.minus("a");
    a.minus("a");
    println!("get_max: {}", a.get_max());
    println!("get_min: {}", a.get_min());
}
