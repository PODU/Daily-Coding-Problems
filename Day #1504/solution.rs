// KMP string matching. Returns first occurrence index, or -1 (meaning "not found"/False).
// Time O(N+k), Space O(k).

fn kmp(text: &str, pat: &str) -> i64 {
    let t: Vec<u8> = text.bytes().collect();
    let p: Vec<u8> = pat.bytes().collect();
    let (n, k) = (t.len(), p.len());
    if k == 0 {
        return 0;
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
                return (i - j) as i64;
            }
        } else if j != 0 {
            j = lps[j - 1];
        } else {
            i += 1;
        }
    }
    -1
}

fn main() {
    println!("{}", kmp("abxabcabcaby", "abcaby"));
}
