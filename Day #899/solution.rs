// 24 game, fixed order: recursively split the sequence at each position, combine left/right results with +,-,*,/ (eps for div). O(4^n) over splits; O(n) depth.
const EPS: f64 = 1e-6;

fn solve(nums: &[f64], lo: usize, hi: usize) -> Vec<f64> {
    if hi - lo == 1 {
        return vec![nums[lo]];
    }
    let mut res = Vec::new();
    for mid in (lo + 1)..hi {
        let l = solve(nums, lo, mid);
        let r = solve(nums, mid, hi);
        for &a in &l {
            for &b in &r {
                res.push(a + b);
                res.push(a - b);
                res.push(a * b);
                if b.abs() > EPS {
                    res.push(a / b);
                }
            }
        }
    }
    res
}

fn can_reach_24(input: &[i32]) -> bool {
    let nums: Vec<f64> = input.iter().map(|&x| x as f64).collect();
    solve(&nums, 0, nums.len())
        .into_iter()
        .any(|v| (v - 24.0).abs() < EPS)
}

fn main() {
    let input = [5, 2, 7, 8];
    println!("{}", if can_reach_24(&input) { "True" } else { "False" });
}
