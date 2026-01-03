// Day 843: find all start indices of pattern in string using KMP.
// Build failure table, single scan. O(n+m) time, O(m) space.
fn kmp_search(s: &str, p: &str) -> Vec<usize> {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let (n, m) = (s.len(), p.len());
    let mut res = Vec::new();
    if m == 0 || m > n {
        return res;
    }
    let mut lps = vec![0usize; m];
    let mut len = 0usize;
    let mut i = 1usize;
    while i < m {
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
            if j == m {
                res.push(i - m);
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
    println!("{:?}", kmp_search("abracadabra", "abr")); // [0, 7]
}
