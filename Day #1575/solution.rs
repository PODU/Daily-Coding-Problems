// Cryptarithmetic solver via backtracking over distinct letters. O(10!/(10-k)!) worst, pruned.
use std::collections::{HashMap, HashSet};

fn val(w: &str, assign: &HashMap<char, i64>) -> i64 {
    let mut v = 0;
    for c in w.chars() {
        v = v * 10 + assign[&c];
    }
    v
}

fn bt(
    idx: usize,
    letters: &[char],
    leading: &HashSet<char>,
    assign: &mut HashMap<char, i64>,
    used: &mut [bool; 10],
    w1: &str,
    w2: &str,
    w3: &str,
) -> bool {
    if idx == letters.len() {
        return val(w1, assign) + val(w2, assign) == val(w3, assign);
    }
    let c = letters[idx];
    for d in 0..=9 {
        if used[d] {
            continue;
        }
        if d == 0 && leading.contains(&c) {
            continue;
        }
        used[d] = true;
        assign.insert(c, d as i64);
        if bt(idx + 1, letters, leading, assign, used, w1, w2, w3) {
            return true;
        }
        used[d] = false;
    }
    false
}

fn main() {
    let (w1, w2, w3) = ("SEND", "MORE", "MONEY");
    let mut letters: Vec<char> = Vec::new();
    let mut seen: HashSet<char> = HashSet::new();
    for w in [w1, w2, w3] {
        for c in w.chars() {
            if seen.insert(c) {
                letters.push(c);
            }
        }
    }
    let mut leading: HashSet<char> = HashSet::new();
    for w in [w1, w2, w3] {
        leading.insert(w.chars().next().unwrap());
    }
    let mut assign: HashMap<char, i64> = HashMap::new();
    let mut used = [false; 10];
    bt(0, &letters, &leading, &mut assign, &mut used, w1, w2, w3);
    let parts: Vec<String> = letters
        .iter()
        .map(|c| format!("'{}': {}", c, assign[c]))
        .collect();
    println!("{{{}}}", parts.join(", "));
}
