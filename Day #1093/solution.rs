// Count smaller elements to the right via Fenwick tree + coordinate compression.
// Time O(n log n), Space O(n).
use std::collections::HashMap;

struct Bit {
    t: Vec<i32>,
}
impl Bit {
    fn new(n: usize) -> Self {
        Bit { t: vec![0; n + 1] }
    }
    fn update(&mut self, mut i: usize) {
        while i < self.t.len() {
            self.t[i] += 1;
            i += i & i.wrapping_neg();
        }
    }
    fn query(&self, mut i: usize) -> i32 {
        let mut s = 0;
        while i > 0 {
            s += self.t[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
}

fn count_smaller(a: &[i32]) -> Vec<i32> {
    let mut uniq: Vec<i32> = a.to_vec();
    uniq.sort();
    uniq.dedup();
    let rank: HashMap<i32, usize> = uniq.iter().enumerate().map(|(i, &v)| (v, i + 1)).collect();
    let mut bit = Bit::new(uniq.len());
    let mut res = vec![0; a.len()];
    for i in (0..a.len()).rev() {
        let rk = rank[&a[i]];
        res[i] = bit.query(rk - 1);
        bit.update(rk);
    }
    res
}

fn main() {
    println!("{:?}", count_smaller(&[3, 4, 9, 6, 1]));
}
