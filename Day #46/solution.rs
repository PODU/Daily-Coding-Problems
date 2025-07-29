// Day 46: Longest palindromic substring via Manacher's algorithm.
// Time: O(n), Space: O(n).
fn longest_palindrome(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let orig: Vec<char> = s.chars().collect();
    let mut t: Vec<char> = vec!['^'];
    for &c in &orig {
        t.push('#');
        t.push(c);
    }
    t.push('#');
    t.push('$');
    let n = t.len();
    let mut p = vec![0usize; n];
    let mut c = 0usize;
    let mut r = 0usize;
    for i in 1..n - 1 {
        if i < r {
            p[i] = std::cmp::min(r - i, p[2 * c - i]);
        }
        while t[i + 1 + p[i]] == t[i - 1 - p[i]] {
            p[i] += 1;
        }
        if i + p[i] > r {
            c = i;
            r = i + p[i];
        }
    }
    let (mut max_len, mut center) = (0usize, 0usize);
    for i in 1..n - 1 {
        if p[i] > max_len {
            max_len = p[i];
            center = i;
        }
    }
    let start = (center - max_len) / 2;
    orig[start..start + max_len].iter().collect()
}

fn main() {
    println!("\"{}\"", longest_palindrome("aabcdcb"));
    println!("\"{}\"", longest_palindrome("bananas"));
}
