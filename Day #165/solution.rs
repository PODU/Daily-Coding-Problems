// Day 165: Count smaller elements to the right. Coordinate-compress, then sweep
// right to left with a Fenwick tree (BIT). Time O(n log n), Space O(n).

struct Bit {
    t: Vec<i32>,
}
impl Bit {
    fn new(n: usize) -> Self {
        Bit { t: vec![0; n + 1] }
    }
    fn update(&mut self, mut i: usize, v: i32) {
        while i < self.t.len() {
            self.t[i] += v;
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
    let mut sorted_unique: Vec<i32> = a.to_vec();
    sorted_unique.sort();
    sorted_unique.dedup();
    let mut bit = Bit::new(sorted_unique.len());
    let mut res = vec![0; a.len()];
    for i in (0..a.len()).rev() {
        let rank = sorted_unique.partition_point(|&x| x < a[i]) + 1;
        res[i] = bit.query(rank - 1);
        bit.update(rank, 1);
    }
    res
}

fn main() {
    println!("{:?}", count_smaller(&[3, 4, 9, 6, 1])); // [1, 1, 2, 1, 0]
}
