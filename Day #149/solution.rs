// Range sum via prefix sums: O(n) preprocessing, O(1) per query. sum(i,j) = pre[j]-pre[i].

struct RangeSum {
    pre: Vec<i64>,
}

impl RangeSum {
    fn new(l: &[i64]) -> Self {
        let mut pre = vec![0i64; l.len() + 1];
        for (k, &v) in l.iter().enumerate() {
            pre[k + 1] = pre[k] + v;
        }
        RangeSum { pre }
    }
    fn sum(&self, i: usize, j: usize) -> i64 {
        self.pre[j] - self.pre[i]
    }
}

fn main() {
    let l = vec![1i64, 2, 3, 4, 5];
    let rs = RangeSum::new(&l);
    println!("{}", rs.sum(1, 3));
}
