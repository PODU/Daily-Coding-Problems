// Zigzag print: place each char at advancing column, row bounces 0..k-1..0.
// Build k row buffers, fill columns; rtrim each row. Time O(n*k), Space O(n*k).
fn zigzag(s: &str, k: usize) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut grid = vec![vec![' '; n]; k];
    let mut row: isize = 0;
    let mut dir: isize = 1;
    for col in 0..n {
        grid[row as usize][col] = chars[col];
        if k > 1 {
            if row == 0 {
                dir = 1;
            } else if row == (k as isize) - 1 {
                dir = -1;
            }
            row += dir;
        }
    }
    grid.into_iter()
        .map(|r| r.into_iter().collect::<String>().trim_end().to_string())
        .collect()
}

fn main() {
    for r in zigzag("thisisazigzag", 4) {
        println!("{}", r);
    }
}
