// Word Search: DFS backtracking from each cell, marking visited in-place.
// Time: O(M*N*4^L), Space: O(L) recursion depth.
fn dfs(b: &mut Vec<Vec<char>>, w: &[char], i: i32, j: i32, k: usize) -> bool {
    if k == w.len() {
        return true;
    }
    if i < 0 || j < 0 || i >= b.len() as i32 || j >= b[0].len() as i32 {
        return false;
    }
    let (ui, uj) = (i as usize, j as usize);
    if b[ui][uj] != w[k] {
        return false;
    }
    let tmp = b[ui][uj];
    b[ui][uj] = '#';
    let found = dfs(b, w, i + 1, j, k + 1) || dfs(b, w, i - 1, j, k + 1)
        || dfs(b, w, i, j + 1, k + 1) || dfs(b, w, i, j - 1, k + 1);
    b[ui][uj] = tmp;
    found
}

fn exists(mut b: Vec<Vec<char>>, word: &str) -> bool {
    let w: Vec<char> = word.chars().collect();
    for i in 0..b.len() as i32 {
        for j in 0..b[0].len() as i32 {
            if dfs(&mut b, &w, i, j, 0) {
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
    for w in ["ABCCED", "SEE", "ABCB"] {
        println!("{}: {}", w, exists(board.clone(), w));
    }
}
