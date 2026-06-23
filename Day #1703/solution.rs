// Prefix sums: precompute prefix[k]=sum(L[0:k]); sum(i,j)=prefix[j]-prefix[i].
// Preprocess O(n), query O(1), Space O(n).
struct RangeSum {
    prefix: Vec<i64>,
}

impl RangeSum {
    fn new(l: &[i64]) -> Self {
        let mut prefix = vec![0i64; l.len() + 1];
        for (k, &x) in l.iter().enumerate() {
            prefix[k + 1] = prefix[k] + x;
        }
        RangeSum { prefix }
    }
    fn sum(&self, i: usize, j: usize) -> i64 {
        self.prefix[j] - self.prefix[i]
    }
}

fn main() {
    let l = vec![1i64, 2, 3, 4, 5];
    let rs = RangeSum::new(&l);
    println!("{}", rs.sum(1, 3));
}
