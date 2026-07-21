// Day 1851: Zigzag string across k lines.
// Place char i at row=zigzag(i), col=i in a grid; print rows. O(n*k) build. Space O(n*k).

fn zigzag(s: &str, k: usize) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut grid = vec![vec![' '; n]; k];
    let period = if k <= 1 { 1 } else { 2 * (k - 1) };
    for (i, &ch) in chars.iter().enumerate() {
        let pos = if k <= 1 { 0 } else { i % period };
        let row = if pos < k { pos } else { period - pos };
        grid[row][i] = ch;
    }
    grid.into_iter()
        .map(|r| r.into_iter().collect::<String>().trim_end().to_string())
        .collect()
}

fn main() {
    for line in zigzag("thisisazigzag", 4) {
        println!("{}", line);
    }
}
