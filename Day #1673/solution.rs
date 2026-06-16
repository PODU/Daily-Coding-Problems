// Day 1673: Min wheel rotations from "000" to target avoiding dead ends.
// BFS over <=1000 states, each with 6 neighbors. Time O(1000), Space O(1000).
use std::collections::{HashMap, HashSet, VecDeque};

fn open_lock(target: &str, deadends: &[&str]) -> Option<i32> {
    let dead: HashSet<String> = deadends.iter().map(|s| s.to_string()).collect();
    let start = "000".to_string();
    if dead.contains(&start) || dead.contains(target) {
        return None;
    }
    if start == target {
        return Some(0);
    }
    let mut dist: HashMap<String, i32> = HashMap::new();
    dist.insert(start.clone(), 0);
    let mut q: VecDeque<String> = VecDeque::new();
    q.push_back(start);
    while let Some(cur) = q.pop_front() {
        let cd = dist[&cur];
        let bytes = cur.as_bytes();
        for i in 0..3 {
            for d in [-1i32, 1] {
                let digit = (bytes[i] as i32 - '0' as i32 + d + 10) % 10;
                let mut nb = bytes.to_vec();
                nb[i] = b'0' + digit as u8;
                let nxt = String::from_utf8(nb).unwrap();
                if dead.contains(&nxt) || dist.contains_key(&nxt) {
                    continue;
                }
                dist.insert(nxt.clone(), cd + 1);
                if nxt == target {
                    return Some(cd + 1);
                }
                q.push_back(nxt);
            }
        }
    }
    None
}

fn main() {
    match open_lock("345", &["333"]) {
        Some(r) => println!("{}", r), // 12
        None => println!("None"),
    }
}
