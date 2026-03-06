// Flood fill via BFS from start pixel, recoloring 4-connected same-color region.
// Time: O(rows*cols), Space: O(rows*cols).
use std::collections::VecDeque;

fn flood_fill(g: &mut Vec<Vec<char>>, sr: usize, sc: usize, color: char) {
    let rows = g.len();
    let cols = g[0].len();
    let start = g[sr][sc];
    if start == color {
        return;
    }
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((sr, sc));
    g[sr][sc] = color;
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some((r, c)) = q.pop_front() {
        for &(dr, dc) in dirs.iter() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let (ur, uc) = (nr as usize, nc as usize);
                if g[ur][uc] == start {
                    g[ur][uc] = color;
                    q.push_back((ur, uc));
                }
            }
        }
    }
}

fn main() {
    let mut g: Vec<Vec<char>> = vec![
        vec!['B', 'B', 'W'],
        vec!['W', 'W', 'W'],
        vec!['W', 'W', 'W'],
        vec!['B', 'B', 'B'],
    ];
    flood_fill(&mut g, 2, 2, 'G');
    for row in &g {
        let line: Vec<String> = row.iter().map(|c| c.to_string()).collect();
        println!("{}", line.join(" "));
    }
}
