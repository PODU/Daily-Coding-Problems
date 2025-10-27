// Validate American-style crossword grid: every white cell in a horizontal AND vertical run
// of length >=3, single connected component of white cells, 180-degree rotational symmetry.
// Time: O(N^2), Space: O(N^2).
use std::collections::VecDeque;

fn is_crossword(g: &[&str]) -> bool {
    let n = g.len();
    if n == 0 {
        return false;
    }
    let grid: Vec<Vec<char>> = g.iter().map(|r| r.chars().collect()).collect();

    // Rule 4: rotational symmetry.
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] != grid[n - 1 - i][n - 1 - j] {
                return false;
            }
        }
    }

    // Rules 1 & 2: runs of length >= 3 in both directions.
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] != '.' {
                continue;
            }
            let mut l = j;
            while l > 0 && grid[i][l - 1] == '.' {
                l -= 1;
            }
            let mut r = j;
            while r < n - 1 && grid[i][r + 1] == '.' {
                r += 1;
            }
            if r - l + 1 < 3 {
                return false;
            }
            let mut t = i;
            while t > 0 && grid[t - 1][j] == '.' {
                t -= 1;
            }
            let mut b = i;
            while b < n - 1 && grid[b + 1][j] == '.' {
                b += 1;
            }
            if b - t + 1 < 3 {
                return false;
            }
        }
    }

    // Rule 3: connectivity via BFS.
    let mut total = 0;
    let mut start: Option<(usize, usize)> = None;
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == '.' {
                total += 1;
                if start.is_none() {
                    start = Some((i, j));
                }
            }
        }
    }
    let start = match start {
        None => return true,
        Some(s) => s,
    };
    let mut seen = vec![vec![false; n]; n];
    let mut q = VecDeque::new();
    q.push_back(start);
    seen[start.0][start.1] = true;
    let mut cnt = 0;
    let dirs: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some((x, y)) = q.pop_front() {
        cnt += 1;
        for (dx, dy) in dirs.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[nx][ny] == '.' && !seen[nx][ny] {
                    seen[nx][ny] = true;
                    q.push_back((nx, ny));
                }
            }
        }
    }
    cnt == total
}

fn main() {
    let valid = [".....", ".....", ".....", ".....", "....."];
    let invalid = ["..#..", ".....", "#...#", ".....", "..#.."];
    println!("{}", is_crossword(&valid));
    println!("{}", is_crossword(&invalid));
}
