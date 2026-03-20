// Fewest insertions for palindrome; lexicographically smallest among minima.
// Interval DP with memoized reconstruction. Time/Space O(n^2).
use std::collections::HashMap;

fn build(s: &[u8], i: i64, j: i64, memo: &mut HashMap<(i64, i64), String>) -> String {
    if i > j {
        return String::new();
    }
    if i == j {
        return (s[i as usize] as char).to_string();
    }
    if let Some(v) = memo.get(&(i, j)) {
        return v.clone();
    }
    let res;
    if s[i as usize] == s[j as usize] {
        let ch = s[i as usize] as char;
        res = format!("{}{}{}", ch, build(s, i + 1, j - 1, memo), ch);
    } else {
        let ci = s[i as usize] as char;
        let cj = s[j as usize] as char;
        let left = format!("{}{}{}", ci, build(s, i + 1, j, memo), ci);
        let right = format!("{}{}{}", cj, build(s, i, j - 1, memo), cj);
        res = if left.len() != right.len() {
            if left.len() < right.len() { left } else { right }
        } else if left <= right {
            left
        } else {
            right
        };
    }
    memo.insert((i, j), res.clone());
    res
}

fn make_palindrome(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut memo = HashMap::new();
    build(bytes, 0, bytes.len() as i64 - 1, &mut memo)
}

fn main() {
    println!("{}", make_palindrome("race"));
    println!("{}", make_palindrome("google"));
}
