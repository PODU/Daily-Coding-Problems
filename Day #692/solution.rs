// Day 692: Autocomplete - return all dictionary strings having s as a prefix.
// Approach: Trie. Insert words O(total chars); query walks prefix then DFS to
// collect matches. Query O(|s| + #matches * len).
use std::collections::HashMap;

#[derive(Default)]
struct Node {
    ch: HashMap<char, Node>,
    end: bool,
}

#[derive(Default)]
struct Trie {
    root: Node,
}

impl Trie {
    fn insert(&mut self, w: &str) {
        let mut cur = &mut self.root;
        for c in w.chars() {
            cur = cur.ch.entry(c).or_default();
        }
        cur.end = true;
    }

    fn dfs(n: &Node, cur: &str, out: &mut Vec<String>) {
        if n.end {
            out.push(cur.to_string());
        }
        for (c, nx) in &n.ch {
            Trie::dfs(nx, &format!("{}{}", cur, c), out);
        }
    }

    fn autocomplete(&self, s: &str) -> Vec<String> {
        let mut cur = &self.root;
        for c in s.chars() {
            match cur.ch.get(&c) {
                Some(n) => cur = n,
                None => return vec![],
            }
        }
        let mut out = Vec::new();
        Trie::dfs(cur, s, &mut out);
        out
    }
}

fn main() {
    let mut t = Trie::default();
    for w in ["dog", "deer", "deal"] {
        t.insert(w);
    }
    let mut res = t.autocomplete("de");
    res.sort();
    println!("{:?}", res); // ["deal", "deer"]
}
