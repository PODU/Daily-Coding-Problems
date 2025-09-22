// Day 313: Min moves on a 3-wheel lock from 000 to target, avoiding dead ends.
// BFS over <=1000 states. O(1000) time. Returns None when unreachable.
use std::collections::{HashMap, HashSet, VecDeque};

fn open_lock(target: &str, deadends: &[&str]) -> Option<i32> {
    let dead: HashSet<&str> = deadends.iter().cloned().collect();
    let start = "000".to_string();
    if dead.contains(start.as_str()) { return None; }
    if start == target { return Some(0); }
    let mut dist: HashMap<String, i32> = HashMap::new();
    dist.insert(start.clone(), 0);
    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some(cur) = q.pop_front() {
        let cd = dist[&cur];
        let bytes = cur.as_bytes();
        for i in 0..3 {
            for d in [-1i32, 1] {
                let mut b = bytes.to_vec();
                b[i] = b'0' + (((b[i] - b'0') as i32 + d + 10) % 10) as u8;
                let nx = String::from_utf8(b).unwrap();
                if dead.contains(nx.as_str()) || dist.contains_key(&nx) { continue; }
                dist.insert(nx.clone(), cd + 1);
                if nx == target { return Some(cd + 1); }
                q.push_back(nx);
            }
        }
    }
    None
}

fn main() {
    match open_lock("123", &[]) {
        Some(n) => println!("{}", n), // 6
        None => println!("None"),
    }
}
