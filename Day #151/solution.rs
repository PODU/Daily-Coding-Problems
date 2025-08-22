// Day 151: Flood fill via BFS. Replace target pixel's connected same-colored
// region with new color. Time O(R*C), Space O(R*C).
use std::collections::VecDeque;

fn flood_fill(m: &mut Vec<Vec<char>>, r: usize, c: usize, color: char) {
    let rows = m.len();
    let cols = m[0].len();
    let target = m[r][c];
    if target == color {
        return;
    }
    let mut q = VecDeque::new();
    q.push_back((r as i32, c as i32));
    m[r][c] = color;
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some((x, y)) = q.pop_front() {
        for (dx, dy) in dirs.iter() {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < rows as i32 && ny >= 0 && ny < cols as i32
                && m[nx as usize][ny as usize] == target
            {
                m[nx as usize][ny as usize] = color;
                q.push_back((nx, ny));
            }
        }
    }
}

fn main() {
    let mut m = vec![
        vec!['B', 'B', 'W'],
        vec!['W', 'W', 'W'],
        vec!['W', 'W', 'W'],
        vec!['B', 'B', 'B'],
    ];
    flood_fill(&mut m, 2, 2, 'G');
    for row in &m {
        let line: Vec<String> = row.iter().map(|c| c.to_string()).collect();
        println!("{}", line.join(" "));
    }
}
