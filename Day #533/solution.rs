// Boggle solver: build a trie from the dictionary, DFS each cell over 8 neighbors
// (each cell used once per path), collect words present in the trie.
// Time: O(cells * 8^L) bounded by trie depth; Space: O(total dictionary chars).
use std::collections::BTreeSet;

#[derive(Default)]
struct Trie {
    next: [Option<Box<Trie>>; 26],
    word: bool,
}

fn dfs(
    board: &Vec<Vec<u8>>,
    r: i32,
    c: i32,
    node: &Trie,
    cur: &mut String,
    used: &mut Vec<Vec<bool>>,
    found: &mut BTreeSet<String>,
) {
    let (rows, cols) = (board.len() as i32, board[0].len() as i32);
    if r < 0 || r >= rows || c < 0 || c >= cols || used[r as usize][c as usize] {
        return;
    }
    let ch = board[r as usize][c as usize];
    let nxt = match &node.next[(ch - b'a') as usize] {
        Some(n) => n,
        None => return,
    };
    cur.push(ch as char);
    used[r as usize][c as usize] = true;
    if nxt.word {
        found.insert(cur.clone());
    }
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr != 0 || dc != 0 {
                dfs(board, r + dr, c + dc, nxt, cur, used, found);
            }
        }
    }
    used[r as usize][c as usize] = false;
    cur.pop();
}

fn main() {
    let board: Vec<Vec<u8>> = vec![
        b"oaan".to_vec(),
        b"etae".to_vec(),
        b"ihkr".to_vec(),
        b"iflv".to_vec(),
    ];
    let dict = ["oath", "pea", "eat", "rain"];

    let mut root = Trie::default();
    for w in dict.iter() {
        let mut node = &mut root;
        for &b in w.as_bytes() {
            node = node.next[(b - b'a') as usize].get_or_insert_with(Default::default);
        }
        node.word = true;
    }

    let r = board.len();
    let c = board[0].len();
    let mut used = vec![vec![false; c]; r];
    let mut found: BTreeSet<String> = BTreeSet::new();
    let mut cur = String::new();
    for i in 0..r as i32 {
        for j in 0..c as i32 {
            dfs(&board, i, j, &root, &mut cur, &mut used, &mut found);
        }
    }

    let v: Vec<String> = found.into_iter().collect();
    println!("[{}]", v.join(", "));
}
