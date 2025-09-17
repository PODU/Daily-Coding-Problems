// Bipartite check (2-coloring) via BFS over enemy graph; split into two teams or False.
// Time O(V+E), Space O(V).
use std::collections::VecDeque;

fn two_color(g: &Vec<Vec<usize>>, n: usize) -> Option<Vec<i32>> {
    let mut color = vec![-1i32; n];
    for s in 0..n {
        if color[s] != -1 {
            continue;
        }
        color[s] = 0;
        let mut q = VecDeque::new();
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            for &v in &g[u] {
                if color[v] == -1 {
                    color[v] = color[u] ^ 1;
                    q.push_back(v);
                } else if color[v] == color[u] {
                    return None;
                }
            }
        }
    }
    Some(color)
}

fn fmt_set(s: &[usize]) -> String {
    let parts: Vec<String> = s.iter().map(|x| x.to_string()).collect();
    format!("{{{}}}", parts.join(", "))
}

fn solve(g: &Vec<Vec<usize>>, n: usize) {
    match two_color(g, n) {
        None => println!("False"),
        Some(color) => {
            let mut a = Vec::new();
            let mut b = Vec::new();
            for i in 0..n {
                if color[i] == 0 {
                    a.push(i);
                } else {
                    b.push(i);
                }
            }
            println!("{} and {}", fmt_set(&a), fmt_set(&b));
        }
    }
}

fn main() {
    let g1: Vec<Vec<usize>> = vec![vec![3], vec![2], vec![1, 4], vec![0, 4, 5], vec![2, 3], vec![3]];
    let g2: Vec<Vec<usize>> = vec![vec![3], vec![2], vec![1, 3, 4], vec![0, 2, 4, 5], vec![2, 3], vec![3]];
    solve(&g1, 6);
    solve(&g2, 6);
}
