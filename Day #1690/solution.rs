// Boggle solver: build a trie from the dictionary, DFS 8-directionally from each
// cell following trie edges with a visited mask. O(cells * 8^L) worst, pruned by trie.
use std::collections::{BTreeSet, HashMap};

struct TrieNode {
    children: HashMap<char, TrieNode>,
    end: bool,
}
impl TrieNode {
    fn new() -> Self {
        TrieNode { children: HashMap::new(), end: false }
    }
}

#[allow(clippy::too_many_arguments)]
fn dfs(
    grid: &Vec<Vec<char>>,
    r: usize,
    c: usize,
    node: &TrieNode,
    path: &mut String,
    vis: &mut Vec<Vec<bool>>,
    found: &mut BTreeSet<String>,
) {
    let ch = grid[r][c];
    let nxt = match node.children.get(&ch) {
        Some(n) => n,
        None => return,
    };
    path.push(ch);
    if nxt.end {
        found.insert(path.clone());
    }
    vis[r][c] = true;
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                let (nr, nc) = (nr as usize, nc as usize);
                if !vis[nr][nc] {
                    dfs(grid, nr, nc, nxt, path, vis, found);
                }
            }
        }
    }
    vis[r][c] = false;
    path.pop();
}

fn main() {
    let rows = ["oaan", "etae", "ihkr", "iflv"];
    let dict = ["oath", "pea", "eat", "rain"];

    let mut root = TrieNode::new();
    for w in dict.iter() {
        let mut nd = &mut root;
        for ch in w.chars() {
            nd = nd.children.entry(ch).or_insert_with(TrieNode::new);
        }
        nd.end = true;
    }

    let grid: Vec<Vec<char>> = rows.iter().map(|s| s.chars().collect()).collect();
    let r_len = grid.len();
    let c_len = grid[0].len();
    let mut vis = vec![vec![false; c_len]; r_len];
    let mut found = BTreeSet::new();
    let mut path = String::new();

    for r in 0..r_len {
        for c in 0..c_len {
            dfs(&grid, r, c, &root, &mut path, &mut vis, &mut found);
        }
    }

    for w in &found {
        println!("{}", w);
    }
}
