// Day 181: Minimum palindrome partitioning.
// DP: palindrome table + min-cut DP with reconstruction. Time O(n^2), Space O(n^2).
fn min_palindrome_partition(s: &str) -> Vec<String> {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return vec![];
    }
    let mut pal = vec![vec![false; n]; n];
    for i in 0..n {
        pal[i][i] = true;
    }
    for l in 2..=n {
        for i in 0..=(n - l) {
            let j = i + l - 1;
            if b[i] == b[j] && (l == 2 || pal[i + 1][j - 1]) {
                pal[i][j] = true;
            }
        }
    }
    const INF: i32 = 1 << 30;
    let mut cut = vec![INF; n + 1];
    let mut prev = vec![-1i32; n + 1];
    cut[0] = 0;
    for i in 1..=n {
        for j in 0..i {
            if pal[j][i - 1] && cut[j] + 1 < cut[i] {
                cut[i] = cut[j] + 1;
                prev[i] = j as i32;
            }
        }
    }
    let mut res = Vec::new();
    let mut i = n as i32;
    while i > 0 {
        let j = prev[i as usize];
        res.push(s[j as usize..i as usize].to_string());
        i = j;
    }
    res.reverse();
    res
}

fn fmt(v: &[String]) -> String {
    let inner: Vec<String> = v.iter().map(|x| format!("\"{}\"", x)).collect();
    format!("[{}]", inner.join(", "))
}

fn main() {
    println!("{}", fmt(&min_palindrome_partition("racecarannakayak")));
    println!("{}", fmt(&min_palindrome_partition("abc")));
}
