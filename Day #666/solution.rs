// Day 666: Minimize difference of two subset sums. Subset-sum DP over reachable sums up to total/2,
// pick reachable sum closest to total/2, reconstruct one subset. Time O(n*sum), Space O(n*sum).
fn main() {
    let a = vec![5, 10, 15, 20, 25];
    let n = a.len();
    let total: i32 = a.iter().sum();
    let half = (total / 2) as usize;
    let mut dp = vec![vec![false; half + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = true;
    }
    for i in 1..=n {
        for s in 0..=half {
            dp[i][s] = dp[i - 1][s]
                || (s >= a[i - 1] as usize && dp[i - 1][s - a[i - 1] as usize]);
        }
    }
    let mut best = 0usize;
    for s in (0..=half).rev() {
        if dp[n][s] {
            best = s;
            break;
        }
    }
    let (mut va, mut vb) = (Vec::new(), Vec::new());
    let mut s = best;
    for i in (1..=n).rev() {
        let w = a[i - 1] as usize;
        if s >= w && dp[i - 1][s - w] {
            va.push(a[i - 1]);
            s -= w;
        } else {
            vb.push(a[i - 1]);
        }
    }
    let f = |v: &[i32]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
    println!("{{{}}} and {{{}}}, difference of {}", f(&va), f(&vb), total - 2 * best as i32);
}
