// Validate American crossword grid: each white cell in horiz & vert runs of len>=3, connected, 180-deg symmetric.
// Time O(N^2), Space O(N^2).
use std::collections::VecDeque;

fn is_valid_crossword(g: &Vec<Vec<i32>>) -> bool {
    let n = g.len();
    if n == 0 {
        return false;
    }

    for i in 0..n {
        for j in 0..n {
            if g[i][j] != g[n - 1 - i][n - 1 - j] {
                return false;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if g[i][j] != 0 {
                continue;
            }
            let (mut l, mut r) = (j as i32, j as i32);
            while l - 1 >= 0 && g[i][(l - 1) as usize] == 0 {
                l -= 1;
            }
            while (r + 1) < n as i32 && g[i][(r + 1) as usize] == 0 {
                r += 1;
            }
            if r - l + 1 < 3 {
                return false;
            }
            let (mut t, mut b) = (i as i32, i as i32);
            while t - 1 >= 0 && g[(t - 1) as usize][j] == 0 {
                t -= 1;
            }
            while (b + 1) < n as i32 && g[(b + 1) as usize][j] == 0 {
                b += 1;
            }
            if b - t + 1 < 3 {
                return false;
            }
        }
    }

    let mut total = 0;
    let mut start: Option<(usize, usize)> = None;
    for i in 0..n {
        for j in 0..n {
            if g[i][j] == 0 {
                total += 1;
                if start.is_none() {
                    start = Some((i, j));
                }
            }
        }
    }
    if total == 0 {
        return false;
    }

    let mut vis = vec![vec![false; n]; n];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let (sr, sc) = start.unwrap();
    queue.push_back((sr, sc));
    vis[sr][sc] = true;
    let mut seen = 0;
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some((r, c)) = queue.pop_front() {
        seen += 1;
        for &(dr, dc) in dirs.iter() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if !vis[nr][nc] && g[nr][nc] == 0 {
                    vis[nr][nc] = true;
                    queue.push_back((nr, nc));
                }
            }
        }
    }
    seen == total
}

fn main() {
    let valid = vec![vec![0; 5]; 5];
    println!("{}", if is_valid_crossword(&valid) { "true" } else { "false" });

    let mut invalid = vec![vec![0; 5]; 5];
    invalid[0][0] = 1;
    println!("{}", if is_valid_crossword(&invalid) { "true" } else { "false" });
}
