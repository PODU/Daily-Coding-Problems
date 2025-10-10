// Boolean parenthesization: count ways the expression evaluates to True.
// Interval DP storing #True/#False per substring. Time O(n^3), Space O(n^2).
fn count_true(a: &[&str]) -> i64 {
    let val: Vec<u8> = a.iter().step_by(2).map(|s| s.as_bytes()[0]).collect();
    let op: Vec<u8> = a.iter().skip(1).step_by(2).map(|s| s.as_bytes()[0]).collect();
    let m = val.len();
    let mut t = vec![vec![0i64; m]; m];
    let mut f = vec![vec![0i64; m]; m];
    for i in 0..m {
        if val[i] == b'T' { t[i][i] = 1; } else { f[i][i] = 1; }
    }
    for len in 2..=m {
        for i in 0..=m - len {
            let j = i + len - 1;
            for k in i..j {
                let o = op[k];
                let (lt, lf, rt, rf) = (t[i][k], f[i][k], t[k + 1][j], f[k + 1][j]);
                let tot = (lt + lf) * (rt + rf);
                match o {
                    b'&' => { t[i][j] += lt * rt; f[i][j] += tot - lt * rt; }
                    b'|' => { t[i][j] += tot - lf * rf; f[i][j] += lf * rf; }
                    _ => { t[i][j] += lt * rf + lf * rt; f[i][j] += lt * rt + lf * rf; }
                }
            }
        }
    }
    t[0][m - 1]
}

fn main() {
    let expr = ["F", "|", "T", "&", "T"];
    println!("{}", count_true(&expr));
}
