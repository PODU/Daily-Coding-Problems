// Day 1770: Flood fill via BFS, 4-directional. Replace connected same-colored region.
// Time: O(rows*cols), Space: O(rows*cols) for the queue in worst case.
use std::collections::VecDeque;

fn flood_fill(g: &mut Vec<Vec<char>>, sr: usize, sc: usize, color: char) {
    let r_len = g.len();
    let c_len = g[0].len();
    let target = g[sr][sc];
    if target == color {
        return;
    }
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((sr, sc));
    g[sr][sc] = color;
    while let Some((r, c)) = q.pop_front() {
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if r > 0 { neighbors.push((r - 1, c)); }
        if r + 1 < r_len { neighbors.push((r + 1, c)); }
        if c > 0 { neighbors.push((r, c - 1)); }
        if c + 1 < c_len { neighbors.push((r, c + 1)); }
        for (nr, nc) in neighbors {
            if g[nr][nc] == target {
                g[nr][nc] = color;
                q.push_back((nr, nc));
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
