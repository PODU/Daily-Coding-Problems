// BFS over squares 1..100; from s try rolls 1..6, apply snake/ladder mapping. Time O(N*6), Space O(N).
use std::collections::{HashMap, VecDeque};

fn min_turns() -> i32 {
    let mut jump: HashMap<i32, i32> = HashMap::new();
    let snakes = [(16,6),(48,26),(49,11),(56,53),(62,19),(64,60),(87,24),(93,73),(95,75),(98,78)];
    let ladders = [(1,38),(4,14),(9,31),(21,42),(28,84),(36,44),(51,67),(71,91),(80,100)];
    for &(a, b) in snakes.iter().chain(ladders.iter()) {
        jump.insert(a, b);
    }

    let mut dist = vec![-1i32; 101];
    dist[1] = 0; // start placed on 1; do NOT apply 1->38 here
    let mut q = VecDeque::new();
    q.push_back(1);
    while let Some(s) = q.pop_front() {
        if s == 100 {
            return dist[s as usize];
        }
        for r in 1..=6 {
            let mut nxt = s + r;
            if nxt > 100 {
                continue;
            }
            if let Some(&d) = jump.get(&nxt) {
                nxt = d;
            }
            if dist[nxt as usize] == -1 {
                dist[nxt as usize] = dist[s as usize] + 1;
                q.push_back(nxt);
            }
        }
    }
    dist[100]
}

fn main() {
    println!("Minimum turns: {}", min_turns());
}
