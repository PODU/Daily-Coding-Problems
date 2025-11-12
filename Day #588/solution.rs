// Day 588: Space-efficient SparseArray backed by a HashMap of non-zero entries.
// Approach: store only non-zero indices. Time O(1) avg per op, Space O(#nonzero).
use std::collections::HashMap;

struct SparseArray {
    data: HashMap<usize, i64>,
    size: usize,
}

impl SparseArray {
    fn new() -> Self {
        SparseArray { data: HashMap::new(), size: 0 }
    }

    fn init(&mut self, arr: &[i64], size: usize) {
        self.size = size;
        self.data.clear();
        for (i, &v) in arr.iter().enumerate() {
            if i >= size {
                break;
            }
            if v != 0 {
                self.data.insert(i, v);
            }
        }
    }

    fn set(&mut self, i: usize, val: i64) {
        assert!(i < self.size, "index out of range");
        if val == 0 {
            self.data.remove(&i);
        } else {
            self.data.insert(i, val);
        }
    }

    fn get(&self, i: usize) -> i64 {
        assert!(i < self.size, "index out of range");
        *self.data.get(&i).unwrap_or(&0)
    }
}

fn main() {
    let mut sa = SparseArray::new();
    sa.init(&[0, 0, 0, 5, 0, 0, 9, 0], 8);
    println!("get(3) = {}", sa.get(3)); // 5
    println!("get(6) = {}", sa.get(6)); // 9
    println!("get(0) = {}", sa.get(0)); // 0
    sa.set(1, 42);
    println!("after set(1,42), get(1) = {}", sa.get(1)); // 42
    sa.set(3, 0);
    println!("after set(3,0), get(3) = {}", sa.get(3)); // 0
}
