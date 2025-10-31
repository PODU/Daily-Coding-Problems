// Zigzag: place char i at (zigzag-row, column i) in a k x n grid; print rows. O(n*k).
fn zigzag(s: &str, k: usize) {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut grid = vec![vec![' '; n]; k];
    let mut row: i32 = 0;
    let mut dir: i32 = if k == 1 { 0 } else { 1 };
    for (i, &ch) in chars.iter().enumerate() {
        grid[row as usize][i] = ch;
        if row == 0 {
            dir = 1;
        } else if row == k as i32 - 1 {
            dir = -1;
        }
        row += dir;
    }
    for r in &grid {
        let line: String = r.iter().collect();
        println!("{}", line.trim_end());
    }
}

fn main() {
    zigzag("thisisazigzag", 4);
}
