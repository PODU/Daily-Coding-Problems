// Day 509: Fewest-insertion palindrome with lexicographically earliest result.
// Memoized interval DP building the actual string. Time O(n^3), Space O(n^2).
use std::collections::HashMap;

fn build(s: &[u8], i: i32, j: i32, memo: &mut HashMap<(i32, i32), String>) -> String {
    if i > j {
        return String::new();
    }
    if i == j {
        return (s[i as usize] as char).to_string();
    }
    if let Some(v) = memo.get(&(i, j)) {
        return v.clone();
    }
    let ci = s[i as usize] as char;
    let cj = s[j as usize] as char;
    let res = if s[i as usize] == s[j as usize] {
        format!("{}{}{}", ci, build(s, i + 1, j - 1, memo), cj)
    } else {
        let a = format!("{}{}{}", ci, build(s, i + 1, j, memo), ci);
        let b = format!("{}{}{}", cj, build(s, i, j - 1, memo), cj);
        if a.len() < b.len() {
            a
        } else if b.len() < a.len() {
            b
        } else if a <= b {
            a
        } else {
            b
        }
    };
    memo.insert((i, j), res.clone());
    res
}

fn solve(input: &str) -> String {
    let s = input.as_bytes();
    let n = s.len() as i32;
    if n == 0 {
        return String::new();
    }
    let mut memo: HashMap<(i32, i32), String> = HashMap::new();
    build(s, 0, n - 1, &mut memo)
}

fn main() {
    println!("{}", solve("race"));
    println!("{}", solve("google"));
}
