// Day 134: SparseArray storing only non-zero entries in a hash map.
// init O(n) once, set/get O(1) average. Space O(#nonzero).
use std::collections::HashMap;

struct SparseArray {
    data: HashMap<usize, i32>,
    size: usize,
}

impl SparseArray {
    fn init(arr: &[i32], sz: usize) -> Self {
        let mut data = HashMap::new();
        for (i, &v) in arr.iter().take(sz).enumerate() {
            if v != 0 {
                data.insert(i, v);
            }
        }
        SparseArray { data, size: sz }
    }
    fn set(&mut self, i: usize, val: i32) {
        assert!(i < self.size, "index out of range");
        if val == 0 {
            self.data.remove(&i);
        } else {
            self.data.insert(i, val);
        }
    }
    fn get(&self, i: usize) -> i32 {
        assert!(i < self.size, "index out of range");
        *self.data.get(&i).unwrap_or(&0)
    }
}

fn main() {
    let mut sa = SparseArray::init(&[0, 0, 7, 0, 0, 0, 3, 0], 8);
    println!("{}", sa.get(2)); // 7
    println!("{}", sa.get(0)); // 0
    sa.set(0, 5);
    println!("{}", sa.get(0)); // 5
    sa.set(2, 0);
    println!("{}", sa.get(2)); // 0
}
