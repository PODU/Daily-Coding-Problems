// Day 276: KMP pattern search. Time O(N + k), Space O(k) -- beats O(N*k).
// Returns Some(start index) or None (False).
fn kmp(text: &str, pat: &str) -> Option<usize> {
    let t = text.as_bytes();
    let p = pat.as_bytes();
    let (n, k) = (t.len(), p.len());
    if k == 0 {
        return Some(0);
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
        if t[i] == p[j] {
            i += 1;
            j += 1;
            if j == k {
                return Some(i - j);
            }
        } else if j != 0 {
            j = lps[j - 1];
        } else {
            i += 1;
        }
    }
    None
}

fn main() {
    println!("{:?}", kmp("abxabcabcaby", "abcaby").map(|x| x as i64).unwrap_or(-1)); // 6
    println!("{:?}", kmp("abxabcabcaby", "zzz").map(|x| x as i64).unwrap_or(-1));    // -1
}
