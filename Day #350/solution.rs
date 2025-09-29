// Min perfect squares summing to N via DP, then greedy-largest reconstruction.
// Time O(N*sqrt N), Space O(N).
fn solve(n: usize) -> String {
    let mut dp = vec![usize::MAX; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        let mut s = 1;
        while s * s <= i {
            if dp[i - s * s] != usize::MAX {
                dp[i] = dp[i].min(dp[i - s * s] + 1);
            }
            s += 1;
        }
    }

    // Reconstruct: greedily pick largest s with dp[i - s*s] == dp[i] - 1.
    let mut squares = Vec::new();
    let mut i = n;
    while i > 0 {
        let mut s = (i as f64).sqrt() as usize;
        while s >= 1 {
            if s * s <= i && dp[i - s * s] == dp[i] - 1 {
                squares.push(s * s);
                i -= s * s;
                break;
            }
            s -= 1;
        }
    }

    let parts: Vec<String> = squares.iter().map(|x| x.to_string()).collect();
    format!("{} ({})", dp[n], parts.join(" + "))
}

fn main() {
    for n in [4usize, 17, 18] {
        println!("{}", solve(n));
    }
}
