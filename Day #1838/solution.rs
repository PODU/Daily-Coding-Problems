// Day 1838: Count smaller elements to the right, via a Fenwick tree over compressed values.
// Time O(N log N), Space O(N).

struct Bit {
    t: Vec<i32>,
}
impl Bit {
    fn new(n: usize) -> Self {
        Bit { t: vec![0; n + 1] }
    }
    fn add(&mut self, mut i: usize) {
        while i < self.t.len() {
            self.t[i] += 1;
            i += i & i.wrapping_neg();
        }
    }
    fn sum(&self, mut i: usize) -> i32 {
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
    let mut bit = Bit::new(uniq.len());
    let mut res = vec![0; a.len()];
    for i in (0..a.len()).rev() {
        let r = uniq.partition_point(|&x| x < a[i]) + 1; // 1-indexed rank
        res[i] = bit.sum(r - 1);
        bit.add(r);
    }
    res
}

fn main() {
    println!("{:?}", count_smaller(&[3, 4, 9, 6, 1]));
}
