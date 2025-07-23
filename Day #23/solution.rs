// Shortest path on a grid with walls using BFS (4-directional).
// Time: O(M*N), Space: O(M*N).
use std::collections::VecDeque;

fn shortest_path(board: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) -> Option<i32> {
    let m = board.len();
    let n = board[0].len();
    let mut visited = vec![vec![false; n]; m];
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();
    q.push_back((start.0, start.1, 0));
    visited[start.0][start.1] = true;
    while let Some((r, c, d)) = q.pop_front() {
        if (r, c) == end {
            return Some(d);
        }
        for &(dr, dc) in dirs.iter() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if !visited[nr][nc] && !board[nr][nc] {
                    visited[nr][nc] = true;
                    q.push_back((nr, nc, d + 1));
                }
            }
        }
    }
    None
}

fn main() {
    let f = false;
    let t = true;
    let board = vec![
        vec![f, f, f, f],
        vec![t, t, f, t],
        vec![f, f, f, f],
        vec![f, f, f, f],
    ];
    match shortest_path(&board, (3, 0), (0, 0)) {
        Some(res) => println!("{}", res),
        None => println!("None"),
    }
}
