// Circular lock min moves via BFS over 1000 states; each state has 6 neighbors (3 wheels x +/-1 mod 10). O(states) time/space.
use std::collections::{HashSet, VecDeque};

fn min_moves(target: &str, deadends: &[&str]) -> i32 {
    let dead: HashSet<String> = deadends.iter().map(|s| s.to_string()).collect();
    if dead.contains("000") || dead.contains(target) {
        return -1;
    }
    if target == "000" {
        return 0;
    }
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert("000".to_string());
    let mut queue: VecDeque<(String, i32)> = VecDeque::new();
    queue.push_back(("000".to_string(), 0));
    while let Some((cur, d)) = queue.pop_front() {
        let bytes = cur.as_bytes();
        for i in 0..3 {
            for &delta in &[1, 9] {
                let mut nb = bytes.to_vec();
                nb[i] = b'0' + ((bytes[i] - b'0') as i32 + delta) as u8 % 10;
                let nx = String::from_utf8(nb).unwrap();
                if visited.contains(&nx) || dead.contains(&nx) {
                    continue;
                }
                if nx == target {
                    return d + 1;
                }
                visited.insert(nx.clone());
                queue.push_back((nx, d + 1));
            }
        }
    }
    -1
}

fn main() {
    let deadends = ["100", "020", "001"];
    println!("{}", min_moves("333", &deadends));
}
