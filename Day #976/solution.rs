// Cryptarithmetic 3-word solver (word1 + word2 = word3) via backtracking.
// Time: O(10!/(10-k)!) for k unique letters; Space: O(k).
use std::collections::{HashMap, HashSet};

fn val(w: &str, assign: &HashMap<char, i64>) -> i64 {
    let mut v = 0;
    for ch in w.chars() {
        v = v * 10 + assign[&ch];
    }
    v
}

fn dfs(i: usize, letters: &Vec<char>, leading: &HashSet<char>,
       assign: &mut HashMap<char, i64>, used: &mut [bool; 10],
       a: &str, b: &str, c: &str) -> bool {
    if i == letters.len() {
        return val(a, assign) + val(b, assign) == val(c, assign);
    }
    let ch = letters[i];
    for d in 0..10 {
        if used[d] { continue; }
        if d == 0 && leading.contains(&ch) { continue; }
        used[d] = true;
        assign.insert(ch, d as i64);
        if dfs(i + 1, letters, leading, assign, used, a, b, c) { return true; }
        used[d] = false;
    }
    false
}

fn main() {
    let (a, b, c) = ("SEND", "MORE", "MONEY");
    let mut letters = Vec::new();
    let mut seen = HashSet::new();
    for w in [a, b, c] {
        for ch in w.chars() {
            if seen.insert(ch) { letters.push(ch); }
        }
    }
    let mut leading = HashSet::new();
    leading.insert(a.chars().next().unwrap());
    leading.insert(b.chars().next().unwrap());
    leading.insert(c.chars().next().unwrap());
    let mut assign = HashMap::new();
    let mut used = [false; 10];
    dfs(0, &letters, &leading, &mut assign, &mut used, a, b, c);
    let order = "SENDMORY";
    let parts: Vec<String> = order.chars().map(|ch| format!("'{}': {}", ch, assign[&ch])).collect();
    println!("{{{}}}", parts.join(", "));
}
