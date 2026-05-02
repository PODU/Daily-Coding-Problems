// Snakes & Ladders: BFS over squares 1..100, dice rolls 1..6, apply jumps. Min turns 1->100.
// Time: O(100*6). Space: O(100).
use std::collections::HashMap;
use std::collections::VecDeque;

fn min_turns() -> i32 {
    let pairs = [
        // snakes
        (16, 6), (48, 26), (49, 11), (56, 53), (62, 19), (64, 60), (87, 24), (93, 73), (95, 75), (98, 78),
        // ladders
        (1, 38), (4, 14), (9, 31), (21, 42), (28, 84), (36, 44), (51, 67), (71, 91), (80, 100),
    ];
    let mut jump: HashMap<i32, i32> = HashMap::new();
    for &(a, b) in pairs.iter() {
        jump.insert(a, b);
    }
    let land = |s: i32| -> i32 { *jump.get(&s).unwrap_or(&s) };

    let start = land(1);
    let mut dist = vec![-1i32; 101];
    let mut q: VecDeque<i32> = VecDeque::new();
    dist[start as usize] = 0;
    q.push_back(start);
    while let Some(s) = q.pop_front() {
        if s == 100 {
            return dist[s as usize];
        }
        for d in 1..=6 {
            let mut nxt = s + d;
            if nxt > 100 {
                continue;
            }
            nxt = land(nxt);
            if dist[nxt as usize] == -1 {
                dist[nxt as usize] = dist[s as usize] + 1;
                q.push_back(nxt);
            }
        }
    }
    -1
}

fn main() {
    println!("{}", min_turns());
}
