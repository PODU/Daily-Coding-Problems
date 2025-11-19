// Combination lock: BFS over 1000 states from "000" to target, avoiding deadends.
// Each of 3 wheels has 2 neighbors (+1/-1 mod 10) => 6 neighbors. Time/space O(1000).
use std::collections::{HashSet, VecDeque};

fn open_lock(deadends: &[&str], target: &str, start: &str) -> Option<i32> {
    let dead: HashSet<String> = deadends.iter().map(|s| s.to_string()).collect();
    if dead.contains(start) {
        return None;
    }
    if start == target {
        return Some(0);
    }
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start.to_string());
    let mut q: VecDeque<String> = VecDeque::new();
    q.push_back(start.to_string());
    let mut steps = 0;
    while !q.is_empty() {
        steps += 1;
        for _ in 0..q.len() {
            let cur = q.pop_front().unwrap();
            let bytes = cur.as_bytes();
            for w in 0..3 {
                for d in [1i32, -1i32] {
                    let mut nb = bytes.to_vec();
                    let digit = ((nb[w] - b'0') as i32 + d + 10) % 10;
                    nb[w] = b'0' + digit as u8;
                    let nxt = String::from_utf8(nb).unwrap();
                    if dead.contains(&nxt) || visited.contains(&nxt) {
                        continue;
                    }
                    if nxt == target {
                        return Some(steps);
                    }
                    visited.insert(nxt.clone());
                    q.push_back(nxt);
                }
            }
        }
    }
    None
}

fn main() {
    let r1 = open_lock(&["010", "021"], "020", "000");
    println!("Min moves to open lock (target 020): {}", r1.unwrap());

    let r2 = open_lock(&["001", "010", "100", "009", "090", "900"], "111", "000");
    match r2 {
        Some(v) => println!("Impossible case (target 111): {}", v),
        None => println!("Impossible case (target 111): None"),
    }
}
