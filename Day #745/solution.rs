// Bloom filter: fixed bit array, k hash functions via double hashing.
// check() may report false positives but never false negatives.
// Time: O(k) per add/check, Space: O(m bits).

struct BloomFilter {
    bits: Vec<bool>,
    m: usize,
    k: usize,
}

impl BloomFilter {
    fn new(size: usize, num_hashes: usize) -> Self {
        BloomFilter { bits: vec![false; size], m: size, k: num_hashes }
    }
    fn hashes(&self, s: &str) -> (u64, u64) {
        let mut h1: u64 = 1469598103934665603; // FNV-1a
        for &b in s.as_bytes() {
            h1 ^= b as u64;
            h1 = h1.wrapping_mul(1099511628211);
        }
        let mut h2: u64 = 5381; // djb2
        for &b in s.as_bytes() {
            h2 = (h2 << 5).wrapping_add(h2).wrapping_add(b as u64);
        }
        (h1, h2)
    }
    fn add(&mut self, s: &str) {
        let (h1, h2) = self.hashes(s);
        for i in 0..self.k as u64 {
            let idx = (h1.wrapping_add(i.wrapping_mul(h2)) % self.m as u64) as usize;
            self.bits[idx] = true;
        }
    }
    fn check(&self, s: &str) -> bool {
        let (h1, h2) = self.hashes(s);
        for i in 0..self.k as u64 {
            let idx = (h1.wrapping_add(i.wrapping_mul(h2)) % self.m as u64) as usize;
            if !self.bits[idx] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut bf = BloomFilter::new(1000, 4);
    bf.add("apple");
    bf.add("banana");
    println!("apple: {}", bf.check("apple"));   // true
    println!("banana: {}", bf.check("banana")); // true
    println!("cherry: {}", bf.check("cherry")); // false
}
