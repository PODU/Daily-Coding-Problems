// 24 game (fixed order): interval recursion combining left/right reachable
// values with + - * / using doubles + epsilon.
// Time: O(n^3 * S^2), Space: O(n^2 * S). Here n=4.
fn solve(a: &[i32], l: usize, r: usize) -> Vec<f64> {
    if l == r {
        return vec![a[l] as f64];
    }
    let mut res = Vec::new();
    for m in l..r {
        let left = solve(a, l, m);
        let right = solve(a, m + 1, r);
        for &x in &left {
            for &y in &right {
                res.push(x + y);
                res.push(x - y);
                res.push(x * y);
                if y.abs() > 1e-9 {
                    res.push(x / y);
                }
            }
        }
    }
    res
}

fn can24(a: &[i32]) -> bool {
    solve(a, 0, a.len() - 1).iter().any(|&v| (v - 24.0).abs() < 1e-6)
}

fn main() {
    let a = [5, 2, 7, 8];
    println!("{}", if can24(&a) { "True" } else { "False" });
}
