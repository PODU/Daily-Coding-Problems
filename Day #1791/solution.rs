// Day 1791: All O(1) data structure (plus / minus / get_max / get_min).
//
// Idea: a doubly linked list of "buckets", one per distinct count, kept
// sorted by count. Each bucket owns the set of keys sitting at that count,
// and a HashMap points every key at its current bucket. Incrementing or
// decrementing a key only moves it to a neighbouring bucket, so each
// operation touches a constant number of nodes.
//
// Rust makes pointer-based linked lists painful, so the list lives in a Vec
// arena: buckets are indices, prev/next are indices, and a free list recycles
// removed slots.
use std::collections::{HashMap, HashSet};

struct Bucket {
    count: i64,
    keys: HashSet<String>,
    prev: usize,
    next: usize,
}

struct AllOne {
    buckets: Vec<Bucket>,
    free: Vec<usize>, // recycled arena slots
    head: usize,      // sentinel, count = i64::MIN
    tail: usize,      // sentinel, count = i64::MAX
    key_bucket: HashMap<String, usize>, // key -> arena index of its bucket
}

impl AllOne {
    fn new() -> Self {
        // Slots 0 and 1 are the sentinels; they stay forever.
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

    /// Grab a slot for a new bucket, reusing a freed one when possible.
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

    /// Splice a fresh bucket for `count` right after `node`.
    fn insert_after(&mut self, node: usize, count: i64) -> usize {
        let b = self.alloc(count);
        let nxt = self.buckets[node].next;
        self.buckets[b].prev = node;
        self.buckets[b].next = nxt;
        self.buckets[nxt].prev = b;
        self.buckets[node].next = b;
        b
    }

    /// Unlink an empty bucket and hand its slot back to the free list.
    fn remove(&mut self, b: usize) {
        let (p, n) = (self.buckets[b].prev, self.buckets[b].next);
        self.buckets[p].next = n;
        self.buckets[n].prev = p;
        self.free.push(b);
    }

    fn plus(&mut self, key: &str) {
        if let Some(&cur) = self.key_bucket.get(key) {
            // Existing key: shift it one bucket to the right (count + 1).
            let mut nxt = self.buckets[cur].next;
            let cc = self.buckets[cur].count;
            if nxt == self.tail || self.buckets[nxt].count != cc + 1 {
                // No bucket for count+1 yet, so make one next door.
                nxt = self.insert_after(cur, cc + 1);
            }
            self.buckets[nxt].keys.insert(key.to_string());
            self.key_bucket.insert(key.to_string(), nxt);
            self.buckets[cur].keys.remove(key);
            if self.buckets[cur].keys.is_empty() {
                self.remove(cur); // the bucket we left is empty now
            }
        } else {
            // New key: it belongs in the count-1 bucket at the front.
            let mut first = self.buckets[self.head].next;
            if first == self.tail || self.buckets[first].count != 1 {
                let h = self.head;
                first = self.insert_after(h, 1);
            }
            self.buckets[first].keys.insert(key.to_string());
            self.key_bucket.insert(key.to_string(), first);
        }
    }

    fn minus(&mut self, key: &str) {
        let cur = match self.key_bucket.get(key) {
            Some(&c) => c,
            None => return, // unknown key -> no-op
        };

        self.buckets[cur].keys.remove(key);
        let cc = self.buckets[cur].count;

        if cc == 1 {
            // Count would hit zero, so the key disappears entirely.
            self.key_bucket.remove(key);
        } else {
            // Shift the key one bucket to the left (count - 1).
            let mut prv = self.buckets[cur].prev;
            if prv == self.head || self.buckets[prv].count != cc - 1 {
                let cp = self.buckets[cur].prev;
                prv = self.insert_after(cp, cc - 1);
            }
            self.buckets[prv].keys.insert(key.to_string());
            self.key_bucket.insert(key.to_string(), prv);
        }

        if self.buckets[cur].keys.is_empty() {
            self.remove(cur);
        }
    }

    // Any key from the end buckets is a valid answer; taking the
    // lexicographically smallest keeps the output deterministic.
    fn get_max(&self) -> String {
        let p = self.buckets[self.tail].prev;
        if p == self.head {
            String::new()
        } else {
            self.buckets[p].keys.iter().min().unwrap().clone()
        }
    }

    fn get_min(&self) -> String {
        let n = self.buckets[self.head].next;
        if n == self.tail {
            String::new()
        } else {
            self.buckets[n].keys.iter().min().unwrap().clone()
        }
    }
}

fn main() {
    let mut a = AllOne::new();
    a.plus("apple");
    a.plus("apple");
    a.plus("banana");
    println!("max={} min={}", a.get_max(), a.get_min()); // apple / banana

    a.plus("banana");
    a.plus("banana");
    println!("max={} min={}", a.get_max(), a.get_min()); // banana / apple

    a.minus("apple");
    a.minus("apple");
    println!("max={} min={}", a.get_max(), a.get_min()); // banana / banana
}
