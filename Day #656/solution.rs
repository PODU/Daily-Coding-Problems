// Flood fill via BFS from start pixel; recolor only cells equal to original color.
// Guard against infinite loop when new color == original. Time O(rows*cols), space O(rows*cols).
use std::collections::VecDeque;

fn flood_fill(img: &mut Vec<Vec<char>>, sr: usize, sc: usize, color: char) {
    let rows = img.len() as isize;
    let cols = img[0].len() as isize;
    let orig = img[sr][sc];
    if orig == color {
        return;
    }
    let mut q: VecDeque<(isize, isize)> = VecDeque::new();
    q.push_back((sr as isize, sc as isize));
    img[sr][sc] = color;
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some((r, c)) = q.pop_front() {
        for (dr, dc) in dirs.iter() {
            let nr = r + dr;
            let nc = c + dc;
            if nr >= 0 && nr < rows && nc >= 0 && nc < cols && img[nr as usize][nc as usize] == orig {
                img[nr as usize][nc as usize] = color;
                q.push_back((nr, nc));
            }
        }
    }
}

fn main() {
    let mut img: Vec<Vec<char>> = vec![
        vec!['B', 'B', 'W'],
        vec!['W', 'W', 'W'],
        vec!['W', 'W', 'W'],
        vec!['B', 'B', 'B'],
    ];
    flood_fill(&mut img, 2, 2, 'G');
    for row in &img {
        let line: Vec<String> = row.iter().map(|c| c.to_string()).collect();
        println!("{}", line.join(" "));
    }
}
