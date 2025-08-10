// Day 98: Word search via DFS backtracking from each cell, marking visited cells
// in place. O(M*N*4^L) time, O(L) recursion space.
fn dfs(b: &mut Vec<Vec<char>>, w: &[char], r: i32, c: i32, i: usize) -> bool {
    if i == w.len() {
        return true;
    }
    if r < 0 || r >= b.len() as i32 || c < 0 || c >= b[0].len() as i32 {
        return false;
    }
    let (ru, cu) = (r as usize, c as usize);
    if b[ru][cu] != w[i] {
        return false;
    }
    let saved = b[ru][cu];
    b[ru][cu] = '#';
    let found = dfs(b, w, r + 1, c, i + 1) || dfs(b, w, r - 1, c, i + 1)
        || dfs(b, w, r, c + 1, i + 1) || dfs(b, w, r, c - 1, i + 1);
    b[ru][cu] = saved;
    found
}

fn exists(board: &Vec<Vec<char>>, word: &str) -> bool {
    let w: Vec<char> = word.chars().collect();
    let mut b = board.clone();
    for r in 0..b.len() as i32 {
        for c in 0..b[0].len() as i32 {
            if dfs(&mut b, &w, r, c, 0) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    println!("{}", exists(&board, "ABCCED")); // true
    println!("{}", exists(&board, "SEE"));    // true
    println!("{}", exists(&board, "ABCB"));   // false
}
