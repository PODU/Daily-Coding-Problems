// Day 1430: Space-efficient SparseArray for a mostly-zero array.
// Approach: store only non-zero indices in a HashMap. Time: O(1) avg per op, Space: O(#nonzero).
use std::collections::HashMap;

struct SparseArray {
    data: HashMap<usize, i64>,
    n: usize,
}

impl SparseArray {
    fn new() -> Self {
        SparseArray { data: HashMap::new(), n: 0 }
    }

    fn init(&mut self, arr: &[i64], size: usize) {
        self.n = size;
        self.data.clear();
        for i in 0..size {
            if arr[i] != 0 {
                self.data.insert(i, arr[i]);
            }
        }
    }

    fn set(&mut self, i: usize, val: i64) {
        assert!(i < self.n, "index out of bounds");
        if val == 0 {
            self.data.remove(&i);
        } else {
            self.data.insert(i, val);
        }
    }

    fn get(&self, i: usize) -> i64 {
        assert!(i < self.n, "index out of bounds");
        *self.data.get(&i).unwrap_or(&0)
    }
}

fn main() {
    let mut sa = SparseArray::new();
    sa.init(&[0, 0, 5, 0, 0, 0, 9, 0], 8);
    println!("{}", sa.get(2)); // 5
    println!("{}", sa.get(3)); // 0
    sa.set(3, 7);
    println!("{}", sa.get(3)); // 7
    sa.set(2, 0);
    println!("{}", sa.get(2)); // 0
}
