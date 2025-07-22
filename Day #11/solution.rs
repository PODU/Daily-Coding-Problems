// Autocomplete via Trie: insert all words, walk to prefix node, DFS collect.
// Build: O(total chars); query: O(|prefix| + matches). Results in insertion order.
use std::collections::BTreeMap;

#[derive(Default)]
struct TrieNode {
    ch: BTreeMap<char, TrieNode>,
    order: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode { ch: BTreeMap::new(), order: -1 }
    }
}

struct Trie {
    root: TrieNode,
    counter: i32,
}

impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode::new(), counter: 0 }
    }
    fn insert(&mut self, w: &str) {
        let mut cur = &mut self.root;
        for c in w.chars() {
            cur = cur.ch.entry(c).or_insert_with(TrieNode::new);
        }
        cur.order = self.counter;
        self.counter += 1;
    }
    fn autocomplete(&self, prefix: &str) -> Vec<String> {
        let mut cur = &self.root;
        for c in prefix.chars() {
            match cur.ch.get(&c) {
                Some(n) => cur = n,
                None => return vec![],
            }
        }
        let mut found: Vec<(i32, String)> = Vec::new();
        fn dfs(n: &TrieNode, buf: &mut String, found: &mut Vec<(i32, String)>) {
            if n.order >= 0 {
                found.push((n.order, buf.clone()));
            }
            for (c, child) in &n.ch {
                buf.push(*c);
                dfs(child, buf, found);
                buf.pop();
            }
        }
        let mut buf = prefix.to_string();
        dfs(cur, &mut buf, &mut found);
        found.sort();
        found.into_iter().map(|(_, w)| w).collect()
    }
}

fn main() {
    let mut t = Trie::new();
    for w in ["dog", "deer", "deal"] {
        t.insert(w);
    }
    println!("{:?}", t.autocomplete("de"));
}
