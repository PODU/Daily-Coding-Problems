// Day 1145: Bloom filter - fixed-size bit array, k hashes.
// add/check O(k); check has false positives but never false negatives.
struct BloomFilter {
    bits: Vec<bool>,
    m: usize,
    k: usize,
}

impl BloomFilter {
    fn new(m: usize, k: usize) -> Self {
        BloomFilter { bits: vec![false; m], m, k }
    }
    fn hash_i(&self, s: &str, i: usize) -> usize {
        let mut h: u64 = 1469598103934665603u64 ^ (i as u64 + 1);
        for c in s.bytes() {
            h ^= c as u64;
            h = h.wrapping_mul(1099511628211);
        }
        (h % self.m as u64) as usize
    }
    fn add(&mut self, v: &str) {
        for i in 0..self.k {
            let idx = self.hash_i(v, i);
            self.bits[idx] = true;
        }
    }
    fn check(&self, v: &str) -> bool {
        (0..self.k).all(|i| self.bits[self.hash_i(v, i)])
    }
}

fn main() {
    let mut bf = BloomFilter::new(1000, 4);
    bf.add("apple");
    bf.add("banana");
    println!("{} {} {}", bf.check("apple"), bf.check("banana"), bf.check("cherry"));
    // true true false (likely)
}
