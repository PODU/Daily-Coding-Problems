// Count smaller elements to the right via Fenwick/BIT + coordinate compression.
// Time O(n log n), Space O(n).
struct BIT {
    t: Vec<i32>,
}

impl BIT {
    fn new(n: usize) -> Self {
        BIT { t: vec![0; n + 1] }
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
    let n = a.len();
    let mut uniq: Vec<i32> = a.to_vec();
    uniq.sort();
    uniq.dedup();
    let mut bit = BIT::new(uniq.len());
    let mut res = vec![0; n];
    for i in (0..n).rev() {
        let rank = uniq.partition_point(|&x| x < a[i]) + 1;
        res[i] = bit.query(rank - 1);
        bit.update(rank, 1);
    }
    res
}

fn main() {
    let a = vec![3, 4, 9, 6, 1];
    let res = count_smaller(&a);
    let parts: Vec<String> = res.iter().map(|x| x.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
