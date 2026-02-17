// Boolean parenthesization via interval DP counting True/False groupings. Time O(n^3), Space O(n^2).
fn count_true(expr: &[char]) -> i64 {
    let m = expr.len();
    let n = (m + 1) / 2;
    let mut val = vec![false; n];
    let mut ops = Vec::new();
    for i in 0..m {
        if i % 2 == 0 {
            val[i / 2] = expr[i] == 'T';
        } else {
            ops.push(expr[i]);
        }
    }
    let mut t = vec![vec![0i64; n]; n];
    let mut f = vec![vec![0i64; n]; n];
    for i in 0..n {
        if val[i] {
            t[i][i] = 1;
        } else {
            f[i][i] = 1;
        }
    }
    for len in 2..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            for k in i..j {
                let op = ops[k];
                let (lt, lf, rt, rf) = (t[i][k], f[i][k], t[k + 1][j], f[k + 1][j]);
                match op {
                    '&' => {
                        t[i][j] += lt * rt;
                        f[i][j] += lf * rf + lf * rt + lt * rf;
                    }
                    '|' => {
                        t[i][j] += lt * rt + lt * rf + lf * rt;
                        f[i][j] += lf * rf;
                    }
                    _ => {
                        t[i][j] += lt * rf + lf * rt;
                        f[i][j] += lt * rt + lf * rf;
                    }
                }
            }
        }
    }
    t[0][n - 1]
}

fn main() {
    let expr = vec!['F', '|', 'T', '&', 'T'];
    println!("{}", count_true(&expr));
}
