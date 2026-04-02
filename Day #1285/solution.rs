// Day 1285: Print a string in zigzag form across k lines.
// Place char i at column i, row = triangle-wave of i. Time O(n*k) to render, Space O(n*k).
fn zigzag(s: &str, k: usize) {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if k <= 1 {
        println!("{}", s);
        return;
    }
    let period = 2 * (k - 1);
    let mut grid = vec![vec![' '; n]; k];
    for i in 0..n {
        let pos = i % period;
        let row = if pos < k { pos } else { period - pos };
        grid[row][i] = chars[i];
    }
    for row in &grid {
        let line: String = row.iter().collect();
        println!("{}", line.trim_end());
    }
}

fn main() {
    zigzag("thisisazigzag", 4);
}
