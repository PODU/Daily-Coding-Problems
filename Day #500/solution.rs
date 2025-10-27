// BFS shortest path on a boolean grid (false=walkable, true=wall).
// Time O(M*N), Space O(M*N).
use std::collections::VecDeque;

fn min_steps(board: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) -> Option<i32> {
    let m = board.len();
    let n = board[0].len();
    if board[start.0][start.1] || board[end.0][end.1] {
        return None;
    }
    let mut visited = vec![vec![false; n]; m];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back(start);
    visited[start.0][start.1] = true;
    let mut steps = 0;
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while !q.is_empty() {
        for _ in 0..q.len() {
            let (r, c) = q.pop_front().unwrap();
            if (r, c) == end {
                return Some(steps);
            }
            for (dr, dc) in dirs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if !visited[nr][nc] && !board[nr][nc] {
                        visited[nr][nc] = true;
                        q.push_back((nr, nc));
                    }
                }
            }
        }
        steps += 1;
    }
    None
}

fn main() {
    let t = true;
    let f = false;
    let board = vec![
        vec![f, f, f, f],
        vec![t, t, f, t],
        vec![f, f, f, f],
        vec![f, f, f, f],
    ];
    match min_steps(&board, (3, 0), (0, 0)) {
        Some(s) => println!("{}", s),
        None => println!("None"),
    }
}
