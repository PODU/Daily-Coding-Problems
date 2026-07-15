// Snakes & Ladders minimum turns: BFS over board squares (unweighted shortest path).
// Each square has up to 6 die-roll edges; snakes/ladders redirect the landing square.
// Time: O(100*6). Space: O(100).
use std::collections::{HashMap, VecDeque};

fn min_turns(jump: &HashMap<i32, i32>, size: i32) -> i32 {
    let mut dist = vec![-1i32; (size + 1) as usize];
    dist[1] = 0;
    let mut q = VecDeque::new();
    q.push_back(1); // begin on square 1; jumps trigger only on rolled squares
    while let Some(sq) = q.pop_front() {
        if sq == size {
            return dist[sq as usize];
        }
        for d in 1..=6 {
            let mut nxt = sq + d;
            if nxt > size {
                continue;
            }
            if let Some(&j) = jump.get(&nxt) {
                nxt = j;
            }
            if dist[nxt as usize] == -1 {
                dist[nxt as usize] = dist[sq as usize] + 1;
                q.push_back(nxt);
            }
        }
    }
    dist[size as usize]
}

fn main() {
    let jump: HashMap<i32, i32> = [
        (16, 6), (48, 26), (49, 11), (56, 53), (62, 19), (64, 60), (87, 24), (93, 73), (95, 75), (98, 78),
        (1, 38), (4, 14), (9, 31), (21, 42), (28, 84), (36, 44), (51, 67), (71, 91), (80, 100),
    ]
    .iter()
    .cloned()
    .collect();
    println!("{}", min_turns(&jump, 100)); // 7
}
