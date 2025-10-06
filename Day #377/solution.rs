// Sliding window median via two heaps with lazy deletion.
// Time: O(n log k), Space: O(k).
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct DualHeap {
    small: BinaryHeap<i64>,          // max-heap
    large: BinaryHeap<Reverse<i64>>, // min-heap
    delayed: HashMap<i64, i64>,
    ss: i64,
    ls: i64,
}

impl DualHeap {
    fn new() -> Self {
        DualHeap { small: BinaryHeap::new(), large: BinaryHeap::new(), delayed: HashMap::new(), ss: 0, ls: 0 }
    }
    fn prune_small(&mut self) {
        while let Some(&num) = self.small.peek() {
            if let Some(c) = self.delayed.get(&num).copied() {
                if c > 0 {
                    if c == 1 { self.delayed.remove(&num); } else { self.delayed.insert(num, c - 1); }
                    self.small.pop();
                    continue;
                }
            }
            break;
        }
    }
    fn prune_large(&mut self) {
        while let Some(&Reverse(num)) = self.large.peek() {
            if let Some(c) = self.delayed.get(&num).copied() {
                if c > 0 {
                    if c == 1 { self.delayed.remove(&num); } else { self.delayed.insert(num, c - 1); }
                    self.large.pop();
                    continue;
                }
            }
            break;
        }
    }
    fn balance(&mut self) {
        if self.ss > self.ls + 1 {
            let v = self.small.pop().unwrap();
            self.large.push(Reverse(v));
            self.ss -= 1; self.ls += 1;
            self.prune_small();
        } else if self.ss < self.ls {
            let Reverse(v) = self.large.pop().unwrap();
            self.small.push(v);
            self.ls -= 1; self.ss += 1;
            self.prune_large();
        }
    }
    fn insert(&mut self, num: i64) {
        if self.small.is_empty() || num <= *self.small.peek().unwrap() {
            self.small.push(num); self.ss += 1;
        } else {
            self.large.push(Reverse(num)); self.ls += 1;
        }
        self.balance();
    }
    fn erase(&mut self, num: i64) {
        *self.delayed.entry(num).or_insert(0) += 1;
        if num <= *self.small.peek().unwrap() {
            self.ss -= 1;
            if num == *self.small.peek().unwrap() { self.prune_small(); }
        } else {
            self.ls -= 1;
            if num == self.large.peek().unwrap().0 { self.prune_large(); }
        }
        self.balance();
    }
    fn median(&self, k: usize) -> f64 {
        if k % 2 == 1 {
            *self.small.peek().unwrap() as f64
        } else {
            (*self.small.peek().unwrap() as f64 + self.large.peek().unwrap().0 as f64) / 2.0
        }
    }
}

fn fmt_num(x: f64) -> String {
    if x.fract() == 0.0 { format!("{}", x as i64) } else { format!("{}", x) }
}

fn main() {
    let arr = [-1i64, 5, 13, 8, 2, 3, 3, 1];
    let k = 3usize;
    let mut dh = DualHeap::new();
    for i in 0..k { dh.insert(arr[i]); }
    let n = arr.len();
    for i in k..=n {
        let win: Vec<String> = arr[i - k..i].iter().map(|v| v.to_string()).collect();
        println!("{} <- median of [{}]", fmt_num(dh.median(k)), win.join(", "));
        if i < n { dh.insert(arr[i]); dh.erase(arr[i - k]); }
    }
}
