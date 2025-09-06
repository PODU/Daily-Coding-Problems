// Boggle solver: build a Trie of the dictionary, DFS from every cell over 8 neighbours.
// Time: O(cells * 8^L) bounded by Trie; Space: O(dictionary size).
use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
struct Trie {
    next: HashMap<char, Trie>,
    word: Option<String>,
}

fn dfs(board: &mut Vec<Vec<char>>, r: i32, c: i32, node: &Trie, found: &mut BTreeSet<String>) {
    let (rows, cols) = (board.len() as i32, board[0].len() as i32);
    let ch = board[r as usize][c as usize];
    let nx = match node.next.get(&ch) {
        Some(n) => n,
        None => return,
    };
    if let Some(w) = &nx.word {
        found.insert(w.clone());
    }
    board[r as usize][c as usize] = '#';
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let (nr, nc) = (r + dr, c + dc);
            if nr >= 0 && nr < rows && nc >= 0 && nc < cols && board[nr as usize][nc as usize] != '#' {
                dfs(board, nr, nc, nx, found);
            }
        }
    }
    board[r as usize][c as usize] = ch;
}

fn boggle(mut board: Vec<Vec<char>>, dict: &[&str]) -> Vec<String> {
    let mut root = Trie::default();
    for w in dict {
        let mut n = &mut root;
        for ch in w.chars() {
            n = n.next.entry(ch).or_default();
        }
        n.word = Some(w.to_string());
    }
    let mut found = BTreeSet::new();
    let (rows, cols) = (board.len(), board[0].len());
    for r in 0..rows {
        for c in 0..cols {
            dfs(&mut board, r as i32, c as i32, &root, &mut found);
        }
    }
    found.into_iter().collect()
}

fn main() {
    let board: Vec<Vec<char>> = vec![
        "oaan".chars().collect(),
        "etae".chars().collect(),
        "ihkr".chars().collect(),
        "iflv".chars().collect(),
    ];
    let dict = ["oath", "pea", "eat", "rain"];
    let res = boggle(board, &dict);
    let s: Vec<String> = res.iter().map(|w| format!("'{}'", w)).collect();
    println!("[{}]", s.join(", "));
}
