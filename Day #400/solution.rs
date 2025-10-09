// Prefix-sum array P (P[k]=sum of first k elems); sum(i,j)=P[j]-P[i]. Preprocess O(n), query O(1).
struct RangeSum {
    p: Vec<i64>,
}

impl RangeSum {
    fn new(l: &[i64]) -> Self {
        let mut p = vec![0i64; l.len() + 1];
        for (k, &x) in l.iter().enumerate() {
            p[k + 1] = p[k] + x;
        }
        RangeSum { p }
    }
    fn sum(&self, i: usize, j: usize) -> i64 {
        self.p[j] - self.p[i]
    }
}

fn main() {
    let l = vec![1i64, 2, 3, 4, 5];
    let rs = RangeSum::new(&l);
    println!("{}", rs.sum(1, 3));
}
