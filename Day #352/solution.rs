// Validate crossword: rotational symmetry, all white runs (H & V) length>=3, white cells connected. O(N^2).
use std::collections::VecDeque;

fn valid(g: &[&str]) -> bool {
    let grid: Vec<Vec<char>> = g.iter().map(|r| r.chars().collect()).collect();
    let n = grid.len();
    // 1. rotational symmetry
    for i in 0..n {
        for j in 0..n {
            if (grid[i][j] == '#') != (grid[n-1-i][n-1-j] == '#') {
                return false;
            }
        }
    }
    // 2a. horizontal runs >= 3
    for i in 0..n {
        let mut run = 0;
        for j in 0..=n {
            if j < n && grid[i][j] == '.' { run += 1; }
            else { if run > 0 && run < 3 { return false; } run = 0; }
        }
    }
    // 2b. vertical runs >= 3
    for j in 0..n {
        let mut run = 0;
        for i in 0..=n {
            if i < n && grid[i][j] == '.' { run += 1; }
            else { if run > 0 && run < 3 { return false; } run = 0; }
        }
    }
    // 3. connectivity
    let mut whites: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == '.' { whites.push((i, j)); }
        }
    }
    if whites.is_empty() { return true; }
    let mut seen = vec![vec![false; n]; n];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back(whites[0]);
    seen[whites[0].0][whites[0].1] = true;
    let mut cnt = 0;
    let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some((i, j)) = q.pop_front() {
        cnt += 1;
        for (di, dj) in dirs.iter() {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && (ni as usize) < n && nj >= 0 && (nj as usize) < n {
                let (ni, nj) = (ni as usize, nj as usize);
                if grid[ni][nj] == '.' && !seen[ni][nj] {
                    seen[ni][nj] = true;
                    q.push_back((ni, nj));
                }
            }
        }
    }
    cnt == whites.len()
}

fn main() {
    let grid_a = [".....", ".....", ".....", ".....", "....."];
    let grid_b = ["#....", ".....", ".....", ".....", "....."];
    println!("{}", valid(&grid_a));
    println!("{}", valid(&grid_b));
}
