// Day 301: Bloom filter - fixed-size probabilistic set. No false negatives.
// add/check O(k); space O(m) bits.
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct BloomFilter { bits: Vec<bool>, k: usize }

impl BloomFilter {
    fn new(m: usize, k: usize) -> Self { BloomFilter { bits: vec![false; m], k } }
    fn h(&self, v: &str, i: usize) -> usize {
        let mut s1 = DefaultHasher::new();
        v.hash(&mut s1);
        let h1 = s1.finish();
        let mut s2 = DefaultHasher::new();
        format!("{}#salt", v).hash(&mut s2);
        let h2 = s2.finish();
        (h1.wrapping_add((i as u64).wrapping_mul(h2)) % self.bits.len() as u64) as usize
    }
    fn add(&mut self, v: &str) {
        for i in 0..self.k { let idx = self.h(v, i); self.bits[idx] = true; }
    }
    fn check(&self, v: &str) -> bool { (0..self.k).all(|i| self.bits[self.h(v, i)]) }
}

fn main() {
    let mut bf = BloomFilter::new(1000, 4);
    bf.add("apple"); bf.add("banana");
    println!("{}", bf.check("apple"));
    println!("{}", bf.check("banana"));
    println!("{}", bf.check("cherry"));
}
