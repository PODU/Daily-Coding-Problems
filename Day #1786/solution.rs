// Recursively combine adjacent pairs (preserves order, covers all parenthesizations)
// applying +,-,*,/ until one value remains; check |v-24|<1e-6.
// Time O(exponential in n), Space O(n) recursion. Here n=4.
fn solve(nums: &[f64]) -> bool {
    if nums.len() == 1 {
        return (nums[0] - 24.0).abs() < 1e-6;
    }
    for i in 0..nums.len() - 1 {
        let (a, b) = (nums[i], nums[i + 1]);
        let mut results = vec![a + b, a - b, a * b];
        if b.abs() > 1e-9 {
            results.push(a / b);
        }
        for r in results {
            let mut next: Vec<f64> = Vec::with_capacity(nums.len() - 1);
            next.extend_from_slice(&nums[..i]);
            next.push(r);
            next.extend_from_slice(&nums[i + 2..]);
            if solve(&next) {
                return true;
            }
        }
    }
    false
}

fn can_get_24(nums: &[i32]) -> bool {
    let d: Vec<f64> = nums.iter().map(|&x| x as f64).collect();
    solve(&d)
}

fn main() {
    println!("{}", if can_get_24(&[5, 2, 7, 8]) { "True" } else { "False" });
}
