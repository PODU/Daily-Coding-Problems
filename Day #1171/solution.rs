// Day 1171: Validate an American-style crossword grid.
// Checks rotational symmetry, white-square connectivity (BFS), and that every
// maximal horizontal/vertical white run has length >= 3.
// Time O(N^2), Space O(N^2).  '#' = black, '.' = white.
use std::collections::VecDeque;

fn is_valid_crossword(g: &[&str]) -> bool {
    let n = g.len();
    if n == 0 {
        return false;
    }
    let grid: Vec<Vec<u8>> = g.iter().map(|r| r.bytes().collect()).collect();
    if grid.iter().any(|r| r.len() != n) {
        return false;
    }
    let white = |i: usize, j: usize| grid[i][j] == b'.';

    // 1. Rotational symmetry.
    for i in 0..n {
        for j in 0..n {
            if white(i, j) != white(n - 1 - i, n - 1 - j) {
                return false;
            }
        }
    }

    // 2. Horizontal runs >= 3.
    for i in 0..n {
        let mut run = 0;
        for j in 0..=n {
            if j < n && white(i, j) {
                run += 1;
            } else {
                if run > 0 && run < 3 {
                    return false;
                }
                run = 0;
            }
        }
    }
    // 3. Vertical runs >= 3.
    for j in 0..n {
        let mut run = 0;
        for i in 0..=n {
            if i < n && white(i, j) {
                run += 1;
            } else {
                if run > 0 && run < 3 {
                    return false;
                }
                run = 0;
            }
        }
    }

    // 4. Connectivity.
    let mut whites = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if white(i, j) {
                whites.push((i, j));
            }
        }
    }
    if whites.is_empty() {
        return true;
    }
    let mut vis = vec![vec![false; n]; n];
    let mut q = VecDeque::new();
    let start = whites[0];
    vis[start.0][start.1] = true;
    q.push_back(start);
    let mut seen = 1usize;
    while let Some((x, y)) = q.pop_front() {
        let nbrs = [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];
        for &(nx, ny) in nbrs.iter() {
            if nx < n && ny < n && !vis[nx][ny] && white(nx, ny) {
                vis[nx][ny] = true;
                seen += 1;
                q.push_back((nx, ny));
            }
        }
    }
    seen == whites.len()
}

fn main() {
    let g1 = [".....", ".....", ".....", ".....", "....."];
    let g2 = [".#...", ".....", ".....", ".....", "...#."];
    println!("{}", if is_valid_crossword(&g1) { "true" } else { "false" });
    println!("{}", if is_valid_crossword(&g2) { "true" } else { "false" });
}
