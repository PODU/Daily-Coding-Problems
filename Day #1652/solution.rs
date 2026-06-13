// Shortest palindrome by insertions, lexicographically earliest: memoized DP on
// (i,j) building best palindrome for s[i..j]. Time O(n^2) states, Space O(n^2).
use std::collections::HashMap;

fn solve(s: &[u8], i: i32, j: i32, memo: &mut HashMap<(i32, i32), String>) -> String {
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
        format!("{}{}{}", ci, solve(s, i + 1, j - 1, memo), ci)
    } else {
        let opt1 = format!("{}{}{}", ci, solve(s, i + 1, j, memo), ci);
        let opt2 = format!("{}{}{}", cj, solve(s, i, j - 1, memo), cj);
        if opt1.len() < opt2.len() {
            opt1
        } else if opt2.len() < opt1.len() {
            opt2
        } else if opt1 <= opt2 {
            opt1
        } else {
            opt2
        }
    };
    memo.insert((i, j), res.clone());
    res
}

fn shortest_palindrome(input: &str) -> String {
    let s = input.as_bytes();
    let mut memo = HashMap::new();
    solve(s, 0, s.len() as i32 - 1, &mut memo)
}

fn main() {
    println!("{}", shortest_palindrome("race"));
    println!("{}", shortest_palindrome("google"));
}
