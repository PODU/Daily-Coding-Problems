// Day 1762: Count parenthesizations of a boolean expression evaluating to True.
// Interval DP over operands: t[i][j]/f[i][j] = #ways range evals True/False,
// combine across each split operator. Time O(n^3), Space O(n^2).
fn count_true(tokens: &[&str]) -> i64 {
    let mut vals: Vec<u8> = Vec::new();
    let mut ops: Vec<u8> = Vec::new();
    for (i, tk) in tokens.iter().enumerate() {
        if i % 2 == 0 {
            vals.push(tk.as_bytes()[0]);
        } else {
            ops.push(tk.as_bytes()[0]);
        }
    }
    let n = vals.len();
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
        for i in 0..=(n - len) {
            let j = i + len - 1;
            for k in i..j {
                let (lt, lf, rt, rf) = (t[i][k], f[i][k], t[k + 1][j], f[k + 1][j]);
                let tot = (lt + lf) * (rt + rf);
                match ops[k] {
                    b'&' => {
                        t[i][j] += lt * rt;
                        f[i][j] += tot - lt * rt;
                    }
                    b'|' => {
                        f[i][j] += lf * rf;
                        t[i][j] += tot - lf * rf;
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
    println!("{}", count_true(&["F", "|", "T", "&", "T"]));
}
