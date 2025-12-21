// Day 772: Boggle solver. Trie of dictionary + DFS from each cell over 8 neighbors,
// marking visited. O(W) to build trie; search bounded by trie/board size.
use std::collections::BTreeSet;

#[derive(Default)]
struct Node {
    ch: [Option<Box<Node>>; 26],
    end: bool,
}

fn insert(root: &mut Node, w: &str) {
    let mut cur = root;
    for b in w.bytes() {
        let i = (b - b'a') as usize;
        cur = cur.ch[i].get_or_insert_with(|| Box::new(Node::default()));
    }
    cur.end = true;
}

fn dfs(b: &mut Vec<Vec<u8>>, r: usize, c: usize, node: &Node, path: &mut Vec<u8>, out: &mut BTreeSet<String>) {
    let ch = b[r][c];
    if ch == b'#' {
        return;
    }
    let nxt = match &node.ch[(ch - b'a') as usize] {
        Some(n) => n,
        None => return,
    };
    path.push(ch);
    if nxt.end {
        out.insert(String::from_utf8(path.clone()).unwrap());
    }
    b[r][c] = b'#';
    let (rows, cols) = (b.len() as i32, b[0].len() as i32);
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                dfs(b, nr as usize, nc as usize, nxt, path, out);
            }
        }
    }
    b[r][c] = ch;
    path.pop();
}

fn solve_boggle(board: &[&str], dict: &[&str]) -> Vec<String> {
    let mut root = Node::default();
    for w in dict {
        insert(&mut root, w);
    }
    let mut b: Vec<Vec<u8>> = board.iter().map(|r| r.bytes().collect()).collect();
    let mut out = BTreeSet::new();
    let mut path = Vec::new();
    for r in 0..b.len() {
        for c in 0..b[0].len() {
            dfs(&mut b, r, c, &root, &mut path, &mut out);
        }
    }
    out.into_iter().collect()
}

fn main() {
    let board = ["oaan", "etae", "ihkr", "iflv"];
    let dict = ["oath", "pea", "eat", "rain"];
    println!("{}", solve_boggle(&board, &dict).join(" ")); // eat oath
}
