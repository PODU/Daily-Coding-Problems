// Day 211: All occurrences of pattern in string via KMP.
// Approach: Knuth-Morris-Pratt with LPS failure function. Time O(n+m), Space O(m).
fn find_occurrences(s: &str, p: &str) -> Vec<usize> {
    let sb = s.as_bytes();
    let pb = p.as_bytes();
    let (m, n) = (pb.len(), sb.len());
    let mut res = Vec::new();
    if m == 0 || m > n {
        return res;
    }
    let mut lps = vec![0usize; m];
    let mut len = 0;
    let mut i = 1;
    while i < m {
        if pb[i] == pb[len] {
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
        if sb[i] == pb[j] {
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
    println!("{:?}", find_occurrences("abracadabra", "abr"));
}
