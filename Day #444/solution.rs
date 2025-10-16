// Day 444: KMP string matching in O(N + k) time, O(k) space (beats O(N*k)).
// Returns the start index of the first match, or None (False) if absent.

fn build_lps(p: &[u8]) -> Vec<usize> {
    let mut lps = vec![0usize; p.len()];
    let mut len = 0usize;
    let mut i = 1usize;
    while i < p.len() {
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
    lps
}

fn kmp_search(text: &str, pat: &str) -> Option<usize> {
    if pat.is_empty() {
        return Some(0);
    }
    let t = text.as_bytes();
    let p = pat.as_bytes();
    let lps = build_lps(p);
    let (mut i, mut j) = (0usize, 0usize);
    while i < t.len() {
        if t[i] == p[j] {
            i += 1;
            j += 1;
            if j == p.len() {
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
    match kmp_search("abxabcabcaby", "abcaby") {
        Some(idx) => println!("{}", idx), // 6
        None => println!("False"),
    }
}
