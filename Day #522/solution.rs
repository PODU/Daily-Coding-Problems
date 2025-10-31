// KMP: build prefix-function for pattern, scan text. O(n+m) time, O(m) space.
fn kmp_search(s: &str, p: &str) -> Vec<usize> {
    let s: Vec<u8> = s.bytes().collect();
    let p: Vec<u8> = p.bytes().collect();
    let m = p.len();
    let mut lps = vec![0usize; m];
    let mut len = 0;
    let mut i = 1;
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
    let mut res = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    while i < s.len() {
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
    println!("{:?}", kmp_search("abracadabra", "abr"));
}
