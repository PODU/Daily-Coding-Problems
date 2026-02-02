// Day 1004: Range sum query sum(i, j) = L[i:j].
// Pre-process a prefix-sum array (O(N)); each query is prefix[j]-prefix[i] in O(1).
struct RangeSum {
    prefix: Vec<i64>,
}

impl RangeSum {
    fn new(l: &[i64]) -> Self {
        let mut prefix = vec![0i64; l.len() + 1];
        for i in 0..l.len() {
            prefix[i + 1] = prefix[i] + l[i];
        }
        RangeSum { prefix }
    }
    fn sum(&self, i: usize, j: usize) -> i64 {
        self.prefix[j] - self.prefix[i]
    }
}

fn main() {
    let rs = RangeSum::new(&[1, 2, 3, 4, 5]);
    println!("{}", rs.sum(1, 3)); // 5
}
