// Min subset-sum difference: 0/1 subset-sum DP over reachable sums, pick best<=total/2; backtrack subset. O(n*total) time/space.
fn format(v: &[i32]) -> String {
    let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn main() {
    let a = [5, 10, 15, 20, 25];
    let n = a.len();
    let total: i32 = a.iter().sum();
    let total_u = total as usize;

    // dp[i][s] = sum s reachable using first i items
    let mut dp = vec![vec![false; total_u + 1]; n + 1];
    dp[0][0] = true;
    for i in 1..=n {
        for s in 0..=total_u {
            dp[i][s] = dp[i - 1][s];
            let ai = a[i - 1] as usize;
            if s >= ai && dp[i - 1][s - ai] {
                dp[i][s] = true;
            }
        }
    }

    let mut best = 0usize;
    for s in (0..=(total_u / 2)).rev() {
        if dp[n][s] {
            best = s;
            break;
        }
    }

    // Backtrack from last item to first to recover subset A
    let mut subset_a: Vec<i32> = Vec::new();
    let mut used = vec![false; n];
    let mut s = best;
    for i in (1..=n).rev() {
        let ai = a[i - 1] as usize;
        if s >= ai && dp[i - 1][s - ai] {
            subset_a.push(a[i - 1]);
            used[i - 1] = true;
            s -= ai;
        }
    }
    subset_a.sort();

    let mut subset_b: Vec<i32> = Vec::new();
    for i in 0..n {
        if !used[i] {
            subset_b.push(a[i]);
        }
    }

    println!("Minimum difference: {}", total - 2 * best as i32);
    println!("Subset A: {}", format(&subset_a));
    println!("Subset B: {}", format(&subset_b));
}
