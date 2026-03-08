// Day 1176: Word search in a 2D board via DFS backtracking.
// Try each cell as a start, explore 4 neighbors, mark visited in-place.
// Time O(M*N*4^L), Space O(L) recursion (L = word length).

fn dfs(b: &mut Vec<Vec<u8>>, w: &[u8], k: usize, i: i32, j: i32) -> bool {
    if k == w.len() {
        return true;
    }
    if i < 0 || i >= b.len() as i32 || j < 0 || j >= b[0].len() as i32 {
        return false;
    }
    let (ui, uj) = (i as usize, j as usize);
    if b[ui][uj] != w[k] {
        return false;
    }
    let saved = b[ui][uj];
    b[ui][uj] = b'#';
    let found = dfs(b, w, k + 1, i + 1, j)
        || dfs(b, w, k + 1, i - 1, j)
        || dfs(b, w, k + 1, i, j + 1)
        || dfs(b, w, k + 1, i, j - 1);
    b[ui][uj] = saved;
    found
}

fn exists(board: &[Vec<u8>], word: &str) -> bool {
    let mut b: Vec<Vec<u8>> = board.to_vec();
    let w = word.as_bytes();
    for i in 0..b.len() {
        for j in 0..b[0].len() {
            if dfs(&mut b, w, 0, i as i32, j as i32) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let board: Vec<Vec<u8>> = vec![
        b"ABCE".to_vec(),
        b"SFCS".to_vec(),
        b"ADEE".to_vec(),
    ];
    println!("{}", exists(&board, "ABCCED"));
    println!("{}", exists(&board, "SEE"));
    println!("{}", exists(&board, "ABCB"));
}
