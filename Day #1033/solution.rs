// Day 1033: Minimum subset-sum difference (partition into two subsets).
// Boolean subset-sum DP over reachable sums up to total/2; answer total-2*best. O(n*sum) time, O(sum) space.
fn min_diff(a: &[i32]) -> i32 {
    let total: i32 = a.iter().sum();
    let half = (total / 2) as usize;
    let mut dp = vec![false; half + 1];
    dp[0] = true;
    for &x in a {
        let x = x as usize;
        for s in (x..=half).rev() {
            if dp[s - x] {
                dp[s] = true;
            }
        }
    }
    for s in (0..=half).rev() {
        if dp[s] {
            return total - 2 * s as i32;
        }
    }
    total
}

fn main() {
    println!("{}", min_diff(&[5, 10, 15, 20, 25]));
}
