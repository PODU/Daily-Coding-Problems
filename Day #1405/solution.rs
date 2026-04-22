// Interval DP: T[i][j]/F[i][j] = #ways subexpr of operands i..j is True/False.
// Split at each operator, combine child counts per &,|,^. Time O(n^3), Space O(n^2).

fn count_true(expr: &[&str]) -> i64 {
    let mut vals = Vec::new();
    let mut ops = Vec::new();
    for (i, e) in expr.iter().enumerate() {
        if i % 2 == 0 {
            vals.push(e.as_bytes()[0]);
        } else {
            ops.push(e.as_bytes()[0]);
        }
    }
    let n = vals.len();
    if n == 0 {
        return 0;
    }
    let mut t = vec![vec![0i64; n]; n];
    let mut f = vec![vec![0i64; n]; n];
    for i in 0..n {
        if vals[i] == b'T' {
            t[i][i] = 1;
        } else {
            f[i][i] = 1;
        }
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            for k in i..j {
                let op = ops[k];
                let (lt, lf, rt, rf) = (t[i][k], f[i][k], t[k + 1][j], f[k + 1][j]);
                let tot = (lt + lf) * (rt + rf);
                let tt = match op {
                    b'&' => lt * rt,
                    b'|' => lt * rt + lt * rf + lf * rt,
                    _ => lt * rf + lf * rt,
                };
                t[i][j] += tt;
                f[i][j] += tot - tt;
            }
        }
    }
    t[0][n - 1]
}

fn main() {
    println!("{}", count_true(&["F", "|", "T", "&", "T"])); // 2
}
