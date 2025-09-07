// Snakes and Ladders: BFS over squares 1..100, each die roll (1..6) is one edge; apply jumps.
// Time: O(100 * 6), Space: O(100).
use std::collections::{HashMap, VecDeque};

fn min_turns() -> i32 {
    let pairs = [
        (16, 6), (48, 26), (49, 11), (56, 53), (62, 19), (64, 60), (87, 24), (93, 73), (95, 75), (98, 78),
        (1, 38), (4, 14), (9, 31), (21, 42), (28, 84), (36, 44), (51, 67), (71, 91), (80, 100),
    ];
    let jump: HashMap<i32, i32> = pairs.iter().cloned().collect();
    let apply = |p: i32| *jump.get(&p).unwrap_or(&p);
    let mut dist = vec![-1i32; 101];
    let start = apply(1);
    let mut q = VecDeque::new();
    q.push_back(start);
    dist[start as usize] = 0;
    while let Some(p) = q.pop_front() {
        if p == 100 {
            return dist[p as usize];
        }
        for d in 1..=6 {
            let mut np = p + d;
            if np > 100 {
                continue;
            }
            np = apply(np);
            if dist[np as usize] == -1 {
                dist[np as usize] = dist[p as usize] + 1;
                q.push_back(np);
            }
        }
    }
    dist[100]
}

fn main() {
    println!("Minimum turns: {}", min_turns());
}
