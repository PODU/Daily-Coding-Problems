// Grid shortest path via BFS over walkable cells. Time O(M*N), Space O(M*N).
// Walls are booleans (true=wall). Returns None if no path.
use std::collections::VecDeque;

fn shortest_path(grid: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) -> Option<i32> {
    let m = grid.len();
    let n = grid[0].len();
    if grid[start.0][start.1] || grid[end.0][end.1] {
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
                    if !visited[nr][nc] && !grid[nr][nc] {
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
    const F: bool = false;
    const T: bool = true;
    let grid = vec![
        vec![F, F, F, F],
        vec![T, T, F, T],
        vec![F, F, F, F],
        vec![F, F, F, F],
    ];
    match shortest_path(&grid, (3, 0), (0, 0)) {
        Some(s) => println!("{}", s), // 7
        None => println!("null"),
    }
}
