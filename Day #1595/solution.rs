// Approach: DP min palindrome partition. pal[i][j] table O(n^2), cut[i]=min cuts for prefix,
// then reconstruct one optimal partition. Time O(n^2), Space O(n^2).

fn min_pal_partition(s: &str) -> Vec<String> {
    let b: Vec<char> = s.chars().collect();
    let n = b.len();
    if n == 0 { return vec![]; }
    let mut pal = vec![vec![false; n]; n];
    for i in 0..n { pal[i][i] = true; }
    for len in 2..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            if b[i] == b[j] && (len == 2 || pal[i + 1][j - 1]) {
                pal[i][j] = true;
            }
        }
    }
    let mut cut = vec![0usize; n];
    let mut start = vec![0usize; n];
    for i in 0..n {
        if pal[0][i] { cut[i] = 0; start[i] = 0; continue; }
        let mut best = usize::MAX;
        let mut bj = 0;
        for j in 1..=i {
            if pal[j][i] && cut[j - 1] + 1 < best {
                best = cut[j - 1] + 1;
                bj = j;
            }
        }
        cut[i] = best;
        start[i] = bj;
    }
    let mut res: Vec<String> = Vec::new();
    let mut i = n as isize - 1;
    while i >= 0 {
        let j = start[i as usize];
        res.push(b[j..=(i as usize)].iter().collect());
        i = j as isize - 1;
    }
    res.reverse();
    res
}

fn fmt_list(v: &[String]) -> String {
    let q: Vec<String> = v.iter().map(|x| format!("\"{}\"", x)).collect();
    format!("[{}]", q.join(", "))
}

fn main() {
    println!("{}", fmt_list(&min_pal_partition("racecarannakayak")));
    println!("{}", fmt_list(&min_pal_partition("abc")));
}
