// Cryptarithmetic solver: backtracking over distinct letters with leading-zero pruning.
// Time: O(10!/(10-k)!) over k<=10 distinct letters; Space: O(k).
use std::collections::HashMap;

fn num(w: &str, assign: &HashMap<char, i64>) -> i64 {
    let mut n = 0i64;
    for c in w.chars() {
        n = n * 10 + assign[&c];
    }
    n
}

fn dfs(
    idx: usize,
    order: &Vec<char>,
    leading: &std::collections::HashSet<char>,
    used: &mut [bool; 10],
    assign: &mut HashMap<char, i64>,
    w1: &str,
    w2: &str,
    w3: &str,
) -> bool {
    if idx == order.len() {
        return num(w1, assign) + num(w2, assign) == num(w3, assign);
    }
    for d in 0..10 {
        if used[d] {
            continue;
        }
        if d == 0 && leading.contains(&order[idx]) {
            continue;
        }
        used[d] = true;
        assign.insert(order[idx], d as i64);
        if dfs(idx + 1, order, leading, used, assign, w1, w2, w3) {
            return true;
        }
        used[d] = false;
        assign.remove(&order[idx]);
    }
    false
}

fn main() {
    let (w1, w2, w3) = ("SEND", "MORE", "MONEY");
    let mut order: Vec<char> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for w in [w1, w2, w3] {
        for c in w.chars() {
            if seen.insert(c) {
                order.push(c);
            }
        }
    }
    let mut leading = std::collections::HashSet::new();
    leading.insert(w1.chars().next().unwrap());
    leading.insert(w2.chars().next().unwrap());
    leading.insert(w3.chars().next().unwrap());

    let mut used = [false; 10];
    let mut assign: HashMap<char, i64> = HashMap::new();

    dfs(0, &order, &leading, &mut used, &mut assign, w1, w2, w3);

    let parts: Vec<String> = order
        .iter()
        .map(|c| format!("'{}': {}", c, assign[c]))
        .collect();
    println!("{{{}}}", parts.join(", "));
}
