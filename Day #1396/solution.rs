// KMP substring search: build LPS array, then single scan.
// Time: O(N + k), Space: O(k).

fn kmp_search(s: &str, pat: &str) -> Option<usize> {
    let s: Vec<u8> = s.bytes().collect();
    let p: Vec<u8> = pat.bytes().collect();
    let (n, k) = (s.len(), p.len());
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
        if s[i] == p[j] {
            i += 1;
            j += 1;
            if j == k {
                return Some(i - k);
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
    match kmp_search("abracadabra", "cad") {
        Some(idx) => println!("{}", idx),
        None => println!("False"),
    }
}
