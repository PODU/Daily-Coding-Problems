// Shortest path on a boolean grid (true=wall) via BFS in 4 directions.
// Time: O(M*N), Space: O(M*N). Returns None if unreachable.
use std::collections::VecDeque;

fn shortest_path(board: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let m = board.len();
    let n = board[0].len();
    if board[start.0][start.1] || board[end.0][end.1] {
        return None;
    }
    let mut seen = vec![vec![false; n]; m];
    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    q.push_back((start.0, start.1, 0));
    seen[start.0][start.1] = true;
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some((r, c, steps)) = q.pop_front() {
        if (r, c) == end {
            return Some(steps);
        }
        for (dr, dc) in dirs.iter() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nr >= m as i32 || nc < 0 || nc >= n as i32 {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if seen[nr][nc] || board[nr][nc] {
                continue;
            }
            seen[nr][nc] = true;
            q.push_back((nr, nc, steps + 1));
        }
    }
    None
}

fn main() {
    let (f, t) = (false, true);
    let board = vec![
        vec![f, f, f, f],
        vec![t, t, f, t],
        vec![f, f, f, f],
        vec![f, f, f, f],
    ];
    match shortest_path(&board, (3, 0), (0, 0)) {
        Some(v) => println!("{}", v),
        None => println!("null"),
    }
}
