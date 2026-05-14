// KMP pattern matching: build failure (LPS) array, then scan once collecting all match starts.
// Time: O(N+k), Space: O(k).
fn find_all(s: &str, p: &str) -> Vec<usize> {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let (n, k) = (s.len(), p.len());
    let mut res = Vec::new();
    if k == 0 || k > n {
        return res;
    }
    let mut lps = vec![0usize; k];
    let mut len = 0;
    let mut i = 1;
    while i < k {
        if p[i] == p[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
    let (mut i, mut j) = (0usize, 0usize);
    while i < n {
        if s[i] == p[j] {
            i += 1;
            j += 1;
            if j == k {
                res.push(i - k);
                j = lps[j - 1];
            }
        } else if j != 0 {
            j = lps[j - 1];
        } else {
            i += 1;
        }
    }
    res
}

fn main() {
    let r = find_all("abracadabra", "abr");
    let parts: Vec<String> = r.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
