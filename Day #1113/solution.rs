// Day 1113 - Validate an American-style crossword grid ('#' black, '.' white)
// Approach: 180-degree symmetry, every white cell in across & down run >= 3,
// and white connectivity. Time: O(N^2), Space: O(N^2).

fn is_crossword(grid: &[&str]) -> bool {
    let n = grid.len();
    if n == 0 {
        return false;
    }
    let g: Vec<Vec<u8>> = grid.iter().map(|r| r.bytes().collect()).collect();
    for r in &g {
        if r.len() != n {
            return false;
        }
    }

    for i in 0..n {
        for j in 0..n {
            if (g[i][j] == b'#') != (g[n - 1 - i][n - 1 - j] == b'#') {
                return false;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if g[i][j] == b'.' {
                let mut len = 1i32;
                let mut k = j as i32 - 1;
                while k >= 0 && g[i][k as usize] == b'.' { len += 1; k -= 1; }
                k = j as i32 + 1;
                while (k as usize) < n && g[i][k as usize] == b'.' { len += 1; k += 1; }
                if len < 3 { return false; }
                len = 1;
                k = i as i32 - 1;
                while k >= 0 && g[k as usize][j] == b'.' { len += 1; k -= 1; }
                k = i as i32 + 1;
                while (k as usize) < n && g[k as usize][j] == b'.' { len += 1; k += 1; }
                if len < 3 { return false; }
            }
        }
    }

    let mut whites = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if g[i][j] == b'.' { whites.push((i, j)); }
        }
    }
    if whites.is_empty() {
        return true;
    }

    let mut seen = vec![vec![false; n]; n];
    let mut st = vec![whites[0]];
    seen[whites[0].0][whites[0].1] = true;
    let mut cnt = 1usize;
    let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some((i, j)) = st.pop() {
        for (di, dj) in dirs.iter() {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && (ni as usize) < n && nj >= 0 && (nj as usize) < n
                && g[ni as usize][nj as usize] == b'.' && !seen[ni as usize][nj as usize] {
                seen[ni as usize][nj as usize] = true;
                cnt += 1;
                st.push((ni as usize, nj as usize));
            }
        }
    }
    cnt == whites.len()
}

fn main() {
    let valid = ["...##", ".....", ".....", ".....", "##..."];
    let invalid = [".....", ".....", ".....", ".....", "....#"];
    println!("Grid 1 valid: {}", if is_crossword(&valid) { "True" } else { "False" });
    println!("Grid 2 valid: {}", if is_crossword(&invalid) { "True" } else { "False" });
}
