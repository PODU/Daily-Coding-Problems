// Day 1428: Partition array into two subsets minimizing sum difference.
// Approach: subset-sum DP over half the total; reconstruct one subset.
// Time: O(n * sum), Space: O(n * sum).

fn min_subset_diff(a: &[i32]) -> (i32, Vec<i32>) {
    let n = a.len();
    let total: i32 = a.iter().sum();
    let half = (total / 2) as usize;

    let mut dp = vec![vec![false; half + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = true;
    }
    for i in 1..=n {
        let ai = a[i - 1] as usize;
        for s in 0..=half {
            dp[i][s] = dp[i - 1][s];
            if s >= ai && dp[i - 1][s - ai] {
                dp[i][s] = true;
            }
        }
    }

    let mut best = 0usize;
    for s in (0..=half).rev() {
        if dp[n][s] {
            best = s;
            break;
        }
    }

    let mut subset = Vec::new();
    let mut s = best;
    for i in (1..=n).rev() {
        let ai = a[i - 1] as usize;
        if s >= ai && dp[i - 1][s - ai] {
            subset.push(a[i - 1]);
            s -= ai;
        }
    }

    (total - 2 * best as i32, subset)
}

fn main() {
    let (diff, subset) = min_subset_diff(&[5, 10, 15, 20, 25]);
    println!("Difference: {}", diff); // Difference: 5
    println!("Subset: {:?}", subset);
}
