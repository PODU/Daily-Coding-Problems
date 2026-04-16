// Shortest path on grid with walls via BFS. Time O(M*N), Space O(M*N).
// Returns None if unreachable.
use std::collections::VecDeque;

fn shortest_path(grid: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) -> Option<i32> {
    let (m, n) = (grid.len(), grid[0].len());
    if grid[start.0][start.1] || grid[end.0][end.1] {
        return None;
    }
    let mut dist = vec![vec![-1i32; n]; m];
    dist[start.0][start.1] = 0;
    let mut q = VecDeque::new();
    q.push_back(start);
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some((r, c)) = q.pop_front() {
        if (r, c) == end {
            return Some(dist[r][c]);
        }
        for (dr, dc) in dirs.iter() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if !grid[nr][nc] && dist[nr][nc] == -1 {
                    dist[nr][nc] = dist[r][c] + 1;
                    q.push_back((nr, nc));
                }
            }
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
        Some(d) => println!("{}", d),
        None => println!("null"),
    }
}
