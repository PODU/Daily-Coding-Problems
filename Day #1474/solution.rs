// Day 1474: Boggle solver. Trie of dictionary + DFS from each cell over
// 8-adjacent neighbors (no reuse), pruned by trie prefixes.
// Time O(R*C*8^L) worst case; Space O(total dict chars).
use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
struct Node {
    next: HashMap<u8, usize>,
    word: Option<String>,
}

struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    fn new() -> Self {
        Trie { nodes: vec![Node::default()] }
    }
    fn insert(&mut self, w: &str) {
        let mut cur = 0;
        for &b in w.as_bytes() {
            if let Some(&nx) = self.nodes[cur].next.get(&b) {
                cur = nx;
            } else {
                let id = self.nodes.len();
                self.nodes.push(Node::default());
                self.nodes[cur].next.insert(b, id);
                cur = id;
            }
        }
        self.nodes[cur].word = Some(w.to_string());
    }
}

fn dfs(board: &mut Vec<Vec<u8>>, r: usize, c: usize, trie: &Trie, node: usize,
       found: &mut BTreeSet<String>) {
    let ch = board[r][c];
    if ch == b'*' {
        return;
    }
    let nxt = match trie.nodes[node].next.get(&ch) {
        Some(&n) => n,
        None => return,
    };
    if let Some(w) = &trie.nodes[nxt].word {
        found.insert(w.clone());
    }
    board[r][c] = b'*';
    let (rows, cols) = (board.len() as i32, board[0].len() as i32);
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                dfs(board, nr as usize, nc as usize, trie, nxt, found);
            }
        }
    }
    board[r][c] = ch;
}

fn main() {
    let mut board: Vec<Vec<u8>> = vec![
        b"oaan".to_vec(),
        b"etae".to_vec(),
        b"ihkr".to_vec(),
        b"iflv".to_vec(),
    ];
    let words = ["oath", "pea", "eat", "rain"];
    let mut trie = Trie::new();
    for w in &words {
        trie.insert(w);
    }
    let mut found = BTreeSet::new();
    for r in 0..board.len() {
        for c in 0..board[0].len() {
            dfs(&mut board, r, c, &trie, 0, &mut found);
        }
    }
    let v: Vec<&String> = found.iter().collect();
    println!("{:?}", v); // ["eat", "oath"]
}
