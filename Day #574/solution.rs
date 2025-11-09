// Day 574: Space-efficient bit array backed by 64-bit words.
// set/get are O(1); storage is ceil(size/64) words.

struct BitArray {
    words: Vec<u64>,
    n: usize,
}

impl BitArray {
    fn new(size: usize) -> Self {
        BitArray { words: vec![0u64; (size + 63) / 64], n: size }
    }
    fn set(&mut self, i: usize, val: u8) {
        assert!(i < self.n, "index out of range");
        if val != 0 {
            self.words[i >> 6] |= 1u64 << (i & 63);
        } else {
            self.words[i >> 6] &= !(1u64 << (i & 63));
        }
    }
    fn get(&self, i: usize) -> u8 {
        assert!(i < self.n, "index out of range");
        ((self.words[i >> 6] >> (i & 63)) & 1) as u8
    }
}

fn main() {
    let mut b = BitArray::new(8);
    b.set(0, 1);
    b.set(3, 1);
    println!("get(0) = {}", b.get(0)); // 1
    println!("get(1) = {}", b.get(1)); // 0
    println!("get(3) = {}", b.get(3)); // 1
    b.set(3, 0);
    println!("get(3) = {}", b.get(3)); // 0
}
