// Bloom filter: fixed bit array (1000 bits) + k=3 hashes via double hashing.
// add/check are O(k); space O(m) bits. check has false positives, no false negatives.
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const SIZE: usize = 1000;
const K: usize = 3;

struct BloomFilter {
    bits: [bool; SIZE],
}

impl BloomFilter {
    fn new() -> Self {
        BloomFilter { bits: [false; SIZE] }
    }

    fn base_hashes(&self, v: &str) -> (u64, u64) {
        let mut h1 = DefaultHasher::new();
        v.hash(&mut h1);
        let mut h2 = DefaultHasher::new();
        (v.to_string() + "salt").hash(&mut h2);
        (h1.finish(), h2.finish())
    }

    fn add(&mut self, v: &str) {
        let (h1, h2) = self.base_hashes(v);
        for i in 0..K as u64 {
            let idx = (h1.wrapping_add(i.wrapping_mul(h2)) % SIZE as u64) as usize;
            self.bits[idx] = true;
        }
    }

    fn check(&self, v: &str) -> bool {
        let (h1, h2) = self.base_hashes(v);
        for i in 0..K as u64 {
            let idx = (h1.wrapping_add(i.wrapping_mul(h2)) % SIZE as u64) as usize;
            if !self.bits[idx] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut bf = BloomFilter::new();
    let added = ["apple", "banana", "cherry"];
    for s in &added {
        bf.add(s);
    }

    println!("Added values (expect all true):");
    for s in &added {
        println!("  check({}) = {}", s, bf.check(s));
    }

    println!("Non-added values (expect mostly false):");
    for s in &["date", "elderberry", "fig", "grape"] {
        println!("  check({}) = {}", s, bf.check(s));
    }
}
