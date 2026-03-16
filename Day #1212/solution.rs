// Day 1212: Space-efficient bit array backed by 64-bit words.
// Pack bits into words; set/get use word index and bit offset. Time O(1) per op, Space O(size/64).
struct BitArray {
    words: Vec<u64>,
    _n: usize,
}

impl BitArray {
    fn new(size: usize) -> Self {
        BitArray { words: vec![0u64; (size + 63) / 64], _n: size }
    }
    fn set(&mut self, i: usize, val: u8) {
        if val != 0 {
            self.words[i >> 6] |= 1u64 << (i & 63);
        } else {
            self.words[i >> 6] &= !(1u64 << (i & 63));
        }
    }
    fn get(&self, i: usize) -> u8 {
        ((self.words[i >> 6] >> (i & 63)) & 1) as u8
    }
}

fn main() {
    let mut b = BitArray::new(10);
    b.set(2, 1);
    b.set(7, 1);
    b.set(2, 0);
    println!("{} {} {}", b.get(2), b.get(7), b.get(0)); // 0 1 0
}
