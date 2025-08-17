// Space-efficient bit array: pack bits into u64 words. set/get O(1), space O(size/64).

struct BitArray {
    words: Vec<u64>,
}

impl BitArray {
    fn init(size: usize) -> Self {
        BitArray { words: vec![0u64; (size + 63) / 64] }
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
    let mut b = BitArray::init(10);
    b.set(2, 1);
    b.set(7, 1);
    b.set(7, 0);
    b.set(9, 1);
    println!("{}{}{}{}", b.get(2), b.get(7), b.get(9), b.get(0)); // 1010
}
