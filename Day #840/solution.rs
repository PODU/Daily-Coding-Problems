// Day 840: Print a string in zigzag form across k lines.
// Char i sits at column i; its row follows the zigzag 0,1,..,k-1,k-2,..,1,0,...
// Build k rows of spaces, place each char, print with trailing spaces trimmed. Time O(N*k).

fn zigzag(s: &str, k: usize) -> String {
    if k == 0 {
        return String::new();
    }
    let chars: Vec<char> = s.chars().collect();
    if k == 1 {
        return chars.iter().collect();
    }
    let n = chars.len();
    let mut rows = vec![vec![' '; n]; k];
    let mut row: i32 = 0;
    let mut step: i32 = 1;
    for (i, &ch) in chars.iter().enumerate() {
        rows[row as usize][i] = ch;
        if row == 0 {
            step = 1;
        } else if row == k as i32 - 1 {
            step = -1;
        }
        row += step;
    }
    rows.iter()
        .map(|r| {
            let line: String = r.iter().collect();
            line.trim_end().to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    println!("{}", zigzag("thisisazigzag", 4));
}
