// Day 186: Minimum subset-sum difference (partition problem).
// Subset-sum DP over total, pick best <= total/2, reconstruct. Time O(n*S), Space O(n*S).
fn solve(a: &[i64]) {
    let n = a.len();
    let tot: i64 = a.iter().sum();
    let totu = tot as usize;
    let mut dp = vec![vec![false; totu + 1]; n + 1];
    dp[0][0] = true;
    for i in 1..=n {
        for s in 0..=totu {
            dp[i][s] = dp[i - 1][s] || (s >= a[i - 1] as usize && dp[i - 1][s - a[i - 1] as usize]);
        }
    }
    let mut best = 0usize;
    for s in (0..=totu / 2).rev() {
        if dp[n][s] {
            best = s;
            break;
        }
    }
    let mut sub = Vec::new();
    let mut other = Vec::new();
    let mut s = best;
    for i in (1..=n).rev() {
        let ai = a[i - 1] as usize;
        if s >= ai && dp[i - 1][s - ai] {
            sub.push(a[i - 1]);
            s -= ai;
        } else {
            other.push(a[i - 1]);
        }
    }
    sub.reverse();
    other.reverse();
    let f = |v: &[i64]| {
        let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
        format!("{{{}}}", parts.join(", "))
    };
    println!("{} and {}", f(&sub), f(&other));
}

fn main() {
    solve(&[5, 10, 15, 20, 25]);
}
