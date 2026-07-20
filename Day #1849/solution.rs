// Day 1849: SparseArray storing only non-zero entries in a HashMap.
// init O(N) once, set/get O(1) average. Space O(non-zero count).
use std::collections::HashMap;

struct SparseArray {
    size: usize,
    data: HashMap<usize, i64>,
}

impl SparseArray {
    fn new(arr: &[i64], size: usize) -> Self {
        let mut data = HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            if v != 0 {
                data.insert(i, v);
            }
        }
        SparseArray { size, data }
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
    let mut sa = SparseArray::new(&[0, 0, 5, 0, 0, 0, 9, 0], 8);
    println!("{}", sa.get(2)); // 5
    println!("{}", sa.get(3)); // 0
    sa.set(3, 7);
    println!("{}", sa.get(3)); // 7
    sa.set(2, 0);
    println!("{}", sa.get(2)); // 0
}
