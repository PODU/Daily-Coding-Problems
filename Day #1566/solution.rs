// Longest palindromic substring via Manacher's algorithm. Time O(n), space O(n).
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
    let mut center = 0usize;
    let mut right = 0usize;
    for i in 1..n - 1 {
        let mirror = 2 * center - i;
        if i < right {
            p[i] = std::cmp::min(right - i, p[mirror]);
        }
        while t[i + p[i] + 1] == t[i - p[i] - 1] {
            p[i] += 1;
        }
        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }
    }
    let mut max_len = 0usize;
    let mut center_index = 0usize;
    for i in 1..n - 1 {
        if p[i] > max_len {
            max_len = p[i];
            center_index = i;
        }
    }
    let start = (center_index - max_len) / 2;
    orig[start..start + max_len].iter().collect()
}

fn main() {
    println!("{}", longest_palindrome("aabcdcb"));
}
