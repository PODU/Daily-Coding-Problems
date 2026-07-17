// Word search: DFS backtracking from each cell. O(R*C*4^L) time, O(L) space.
fn dfs(b: &mut Vec<Vec<u8>>, w: &[u8], i: usize, r: i32, c: i32) -> bool {
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
    b[ru][cu] = b'#';
    let found = dfs(b, w, i + 1, r + 1, c)
        || dfs(b, w, i + 1, r - 1, c)
        || dfs(b, w, i + 1, r, c + 1)
        || dfs(b, w, i + 1, r, c - 1);
    b[ru][cu] = saved;
    found
}

fn exists(board: &Vec<Vec<u8>>, w: &str) -> bool {
    let mut b = board.clone();
    let wb = w.as_bytes();
    for r in 0..b.len() {
        for c in 0..b[0].len() {
            if dfs(&mut b, wb, 0, r as i32, c as i32) {
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
    for w in ["ABCCED", "SEE", "ABCB"].iter() {
        let res = if exists(&board, w) { "true" } else { "false" };
        println!("exists(board, \"{}\") returns {}", w, res);
    }
}
