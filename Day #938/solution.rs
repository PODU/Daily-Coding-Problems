// Day 938: Min moves on a 3-wheel lock from 000 to target, avoiding dead ends. BFS over
// 1000 states, each with 6 neighbors. Time O(1000*6), Space O(1000). Returns None if blocked.
use std::collections::{HashSet, VecDeque};

fn min_moves(target: &str, dead_list: &[&str]) -> Option<i32> {
    let dead: HashSet<String> = dead_list.iter().map(|s| s.to_string()).collect();
    let start = String::from("000");
    if dead.contains(&start) || dead.contains(target) {
        return None;
    }
    if start == target {
        return Some(0);
    }
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start.clone());
    let mut q: VecDeque<(String, i32)> = VecDeque::new();
    q.push_back((start, 0));
    while let Some((cur, d)) = q.pop_front() {
        let bytes = cur.as_bytes();
        for i in 0..3 {
            for delta in [1i32, 9] {
                let digit = ((bytes[i] - b'0') as i32 + delta) % 10;
                let mut nb = cur.clone().into_bytes();
                nb[i] = b'0' + digit as u8;
                let nx = String::from_utf8(nb).unwrap();
                if dead.contains(&nx) || visited.contains(&nx) {
                    continue;
                }
                if nx == target {
                    return Some(d + 1);
                }
                visited.insert(nx.clone());
                q.push_back((nx, d + 1));
            }
        }
    }
    None
}

fn main() {
    println!("{}", min_moves("123", &[]).map_or("None".to_string(), |v| v.to_string())); // 6
    let dead = ["100", "900", "010", "090", "001", "009"];
    match min_moves("111", &dead) {
        Some(v) => println!("{}", v),
        None => println!("None"), // None
    }
}
