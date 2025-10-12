// Day 419: Min steps to reduce N to 1 (decrement, or jump to larger factor).
// BFS over values 1..N. Time: O(N*sqrt(N)), Space: O(N).
use std::collections::VecDeque;

fn min_steps(n: usize) -> i64 {
    if n <= 1 {
        return 0;
    }
    let mut dist = vec![-1i64; n + 1];
    dist[n] = 0;
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(n);
    while let Some(v) = q.pop_front() {
        if v == 1 {
            return dist[1];
        }
        if v - 1 >= 1 && dist[v - 1] == -1 {
            dist[v - 1] = dist[v] + 1;
            q.push_back(v - 1);
        }
        let mut a = 2usize;
        while a * a <= v {
            if v % a == 0 {
                let larger = v / a;
                if dist[larger] == -1 {
                    dist[larger] = dist[v] + 1;
                    q.push_back(larger);
                }
            }
            a += 1;
        }
    }
    dist[1]
}

fn main() {
    let n = 100usize;
    println!("{}  (route: 100 -> 10 -> 9 -> 3 -> 2 -> 1)", min_steps(n));
}
