// Day 949: Autocomplete - return all words having query as a prefix, using a Trie.
// Build O(total chars); query O(|s| + matches). Insertion order preserved.
use std::collections::HashMap;

#[derive(Default)]
struct Node {
    next: HashMap<char, usize>,
    ids: Vec<usize>,
}

struct Trie {
    nodes: Vec<Node>,
    words: Vec<String>,
}

impl Trie {
    fn new() -> Self {
        Trie { nodes: vec![Node::default()], words: Vec::new() }
    }
    fn insert(&mut self, w: &str) {
        let idx = self.words.len();
        self.words.push(w.to_string());
        let mut cur = 0;
        for c in w.chars() {
            cur = match self.nodes[cur].next.get(&c) {
                Some(&n) => n,
                None => {
                    let n = self.nodes.len();
                    self.nodes.push(Node::default());
                    self.nodes[cur].next.insert(c, n);
                    n
                }
            };
            self.nodes[cur].ids.push(idx);
        }
    }
    fn with_prefix(&self, s: &str) -> Vec<String> {
        let mut cur = 0;
        for c in s.chars() {
            match self.nodes[cur].next.get(&c) {
                Some(&n) => cur = n,
                None => return Vec::new(),
            }
        }
        self.nodes[cur].ids.iter().map(|&i| self.words[i].clone()).collect()
    }
}

fn main() {
    let mut t = Trie::new();
    for w in ["dog", "deer", "deal"] {
        t.insert(w);
    }
    println!("[{}]", t.with_prefix("de").join(", ")); // [deer, deal]
}
