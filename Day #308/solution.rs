// Day 308: Count parenthesizations evaluating True. Interval DP. O(n^3) time, O(n^2) space.
fn count_true(e: &[&str]) -> i64 {
    let n = e.len();
    let v = (n + 1) / 2;
    let mut t = vec![vec![0i64; v]; v];
    let mut f = vec![vec![0i64; v]; v];
    for i in 0..v {
        if e[2 * i] == "T" { t[i][i] = 1; } else { f[i][i] = 1; }
    }
    for len in 2..=v {
        for i in 0..=(v - len) {
            let j = i + len - 1;
            for k in i..j {
                let op = e[2 * k + 1];
                let (lt, lf, rt, rf) = (t[i][k], f[i][k], t[k + 1][j], f[k + 1][j]);
                let total = (lt + lf) * (rt + rf);
                let tt = match op {
                    "&" => lt * rt,
                    "|" => lt * rt + lt * rf + lf * rt,
                    _ => lt * rf + lf * rt, // ^
                };
                t[i][j] += tt;
                f[i][j] += total - tt;
            }
        }
    }
    t[0][v - 1]
}

fn main() {
    println!("{}", count_true(&["F", "|", "T", "&", "T"])); // 2
}
