// Day 970: Space-efficient SparseArray storing only non-zero entries.
// Approach: HashMap of index->value, default 0. Time O(1) avg per op, Space O(#nonzero).
use std::collections::HashMap;

struct SparseArray {
    m: HashMap<usize, i32>,
    size: usize,
}

impl SparseArray {
    fn init(arr: &[i32], size: usize) -> Self {
        let mut m = HashMap::new();
        for i in 0..size {
            if arr[i] != 0 {
                m.insert(i, arr[i]);
            }
        }
        SparseArray { m, size }
    }
    fn set(&mut self, i: usize, val: i32) {
        assert!(i < self.size, "index out of range");
        if val == 0 {
            self.m.remove(&i);
        } else {
            self.m.insert(i, val);
        }
    }
    fn get(&self, i: usize) -> i32 {
        assert!(i < self.size, "index out of range");
        *self.m.get(&i).unwrap_or(&0)
    }
}

fn main() {
    let mut sa = SparseArray::init(&[0, 0, 5, 0, 0, 0, 9, 0], 8);
    println!("{}", sa.get(2)); // 5
    println!("{}", sa.get(3)); // 0
    sa.set(3, 7);
    println!("{}", sa.get(3)); // 7
    sa.set(2, 0);
    println!("{}", sa.get(2)); // 0
}
