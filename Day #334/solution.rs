// 24 Game (fixed order): recursive split of contiguous list, combine with +,-,*,/ (f64).
// Time: O(1) for fixed 4 numbers. Space: O(1).

fn solve(a: &[f64]) -> Vec<f64> {
    if a.len() == 1 {
        return vec![a[0]];
    }
    let mut res = Vec::new();
    for i in 1..a.len() {
        let l_vals = solve(&a[..i]);
        let r_vals = solve(&a[i..]);
        for &l in &l_vals {
            for &r in &r_vals {
                res.push(l + r);
                res.push(l - r);
                res.push(l * r);
                if r.abs() > 1e-9 {
                    res.push(l / r);
                }
            }
        }
    }
    res
}

fn can24(a: &[f64]) -> bool {
    solve(a).iter().any(|&v| (v - 24.0).abs() < 1e-6)
}

fn main() {
    let nums = [5.0, 2.0, 7.0, 8.0];
    println!("{}", if can24(&nums) { "True" } else { "False" });
}
