// Space-efficient bit array packing 32 bits per word.
// init(size), set(i,val), get(i): each O(1); space O(size/32) words.
struct BitArray {
    words: Vec<u32>,
    #[allow(dead_code)]
    n: usize,
}

impl BitArray {
    fn init(size: usize) -> Self {
        BitArray { words: vec![0; (size + 31) / 32], n: size }
    }
    fn set(&mut self, i: usize, val: u32) {
        if val != 0 {
            self.words[i >> 5] |= 1 << (i & 31);
        } else {
            self.words[i >> 5] &= !(1 << (i & 31));
        }
    }
    fn get(&self, i: usize) -> u32 {
        (self.words[i >> 5] >> (i & 31)) & 1
    }
}

fn main() {
    let mut b = BitArray::init(10);
    b.set(1, 1);
    b.set(4, 1);
    b.set(4, 0);
    b.set(9, 1);
    println!("{} {} {} {}", b.get(1), b.get(4), b.get(9), b.get(0));
    // expected: 1 0 1 0
}
