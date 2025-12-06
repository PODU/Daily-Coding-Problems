// Day 706: 24 Game (fixed order). Try every parenthesization over the fixed
// sequence, combining sub-results with +,-,*,/. Time ~O(1) for 4 numbers.
fn solve(nums: &[f64]) -> Vec<f64> {
    if nums.len() == 1 {
        return vec![nums[0]];
    }
    let mut res = Vec::new();
    for i in 1..nums.len() {
        let lv = solve(&nums[..i]);
        let rv = solve(&nums[i..]);
        for &a in &lv {
            for &b in &rv {
                res.push(a + b);
                res.push(a - b);
                res.push(a * b);
                if b.abs() > 1e-9 {
                    res.push(a / b);
                }
            }
        }
    }
    res
}

fn game24(digits: &[i32]) -> bool {
    let nums: Vec<f64> = digits.iter().map(|&d| d as f64).collect();
    solve(&nums).iter().any(|&v| (v - 24.0).abs() < 1e-6)
}

fn main() {
    println!("{}", if game24(&[5, 2, 7, 8]) { "True" } else { "False" });
}
