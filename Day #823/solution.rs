// Space-efficient bit array using 32-bit words; index>>5 picks word, 1<<(index&31) picks bit.
// Time: O(1) per op, Space: O(n/32 words).

struct BitArray {
    words: Vec<u32>,
}

impl BitArray {
    fn new(size: usize) -> Self {
        BitArray { words: vec![0u32; (size + 31) >> 5] }
    }
    fn set(&mut self, i: usize, val: u8) {
        if val != 0 {
            self.words[i >> 5] |= 1u32 << (i & 31);
        } else {
            self.words[i >> 5] &= !(1u32 << (i & 31));
        }
    }
    fn get(&self, i: usize) -> u32 {
        (self.words[i >> 5] >> (i & 31)) & 1
    }
}

fn main() {
    let mut ba = BitArray::new(16);
    ba.set(0, 1);
    ba.set(5, 1);
    ba.set(15, 1);
    println!("get(0)={}", ba.get(0));
    println!("get(1)={}", ba.get(1));
    println!("get(5)={}", ba.get(5));
    println!("get(15)={}", ba.get(15));
}
