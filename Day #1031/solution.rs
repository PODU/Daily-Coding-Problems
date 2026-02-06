// Day 1031: Snakes & Ladders min turns. BFS over squares 1..100, each turn rolls 1..6,
// applying snake/ladder mapping when a roll lands on a key. Time O(100*6), Space O(100).
use std::collections::HashMap;
use std::collections::VecDeque;

fn min_turns() -> i32 {
    let pairs = [
        (16, 6), (48, 26), (49, 11), (56, 53), (62, 19), (64, 60), (87, 24), (93, 73),
        (95, 75), (98, 78), (1, 38), (4, 14), (9, 31), (21, 42), (28, 84), (36, 44),
        (51, 67), (71, 91), (80, 100),
    ];
    let jumps: HashMap<i32, i32> = pairs.iter().cloned().collect();
    let mut dist = vec![-1i32; 101];
    dist[1] = 0;
    let mut q = VecDeque::new();
    q.push_back(1);
    while let Some(s) = q.pop_front() {
        if s == 100 {
            return dist[s as usize];
        }
        for d in 1..=6 {
            let mut nx = s + d;
            if nx > 100 {
                continue;
            }
            if let Some(&v) = jumps.get(&nx) {
                nx = v;
            }
            if dist[nx as usize] == -1 {
                dist[nx as usize] = dist[s as usize] + 1;
                q.push_back(nx);
            }
        }
    }
    -1
}

fn main() {
    println!("{}", min_turns());
}
