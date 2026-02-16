// Flood fill via BFS from the seed pixel. Time O(R*C), Space O(R*C).
use std::collections::VecDeque;

fn flood_fill(img: &mut Vec<Vec<char>>, sr: usize, sc: usize, c: char) {
    let src = img[sr][sc];
    if src == c {
        return;
    }
    let (rows, cols) = (img.len() as i32, img[0].len() as i32);
    let mut q = VecDeque::new();
    q.push_back((sr as i32, sc as i32));
    img[sr][sc] = c;
    let dirs = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
    while let Some((r, co)) = q.pop_front() {
        for (dr, dc) in dirs.iter() {
            let (nr, nc) = (r + dr, co + dc);
            if nr >= 0 && nr < rows && nc >= 0 && nc < cols
                && img[nr as usize][nc as usize] == src
            {
                img[nr as usize][nc as usize] = c;
                q.push_back((nr, nc));
            }
        }
    }
}

fn main() {
    let mut img = vec![
        vec!['B', 'B', 'W'],
        vec!['W', 'W', 'W'],
        vec!['W', 'W', 'W'],
        vec!['B', 'B', 'B'],
    ];
    flood_fill(&mut img, 2, 2, 'G');
    for row in &img {
        let s: Vec<String> = row.iter().map(|c| c.to_string()).collect();
        println!("{}", s.join(" "));
    }
}
