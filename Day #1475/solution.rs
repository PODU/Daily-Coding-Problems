// Day 1475: Autocomplete via trie. Walk to prefix node, collect subtree words.
// Build O(total chars); query O(len(prefix) + matches). Space O(total chars).
use std::collections::HashMap;

#[derive(Default)]
struct Node {
    next: HashMap<u8, usize>,
    order: i32,
    word: Option<String>,
}

struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    fn new() -> Self {
        let mut n = Node::default();
        n.order = -1;
        Trie { nodes: vec![n] }
    }
    fn insert(&mut self, w: &str, order: i32) {
        let mut cur = 0;
        for &b in w.as_bytes() {
            if let Some(&nx) = self.nodes[cur].next.get(&b) {
                cur = nx;
            } else {
                let id = self.nodes.len();
                let mut nn = Node::default();
                nn.order = -1;
                self.nodes.push(nn);
                self.nodes[cur].next.insert(b, id);
                cur = id;
            }
        }
        self.nodes[cur].order = order;
        self.nodes[cur].word = Some(w.to_string());
    }
    fn collect(&self, node: usize, out: &mut Vec<(i32, String)>) {
        if self.nodes[node].order >= 0 {
            out.push((self.nodes[node].order, self.nodes[node].word.clone().unwrap()));
        }
        for (_, &c) in &self.nodes[node].next {
            self.collect(c, out);
        }
    }
    fn starts_with(&self, prefix: &str) -> Vec<String> {
        let mut cur = 0;
        for &b in prefix.as_bytes() {
            match self.nodes[cur].next.get(&b) {
                Some(&n) => cur = n,
                None => return vec![],
            }
        }
        let mut out = Vec::new();
        self.collect(cur, &mut out);
        out.sort_by_key(|t| t.0);
        out.into_iter().map(|t| t.1).collect()
    }
}

fn main() {
    let mut t = Trie::new();
    for (i, w) in ["dog", "deer", "deal"].iter().enumerate() {
        t.insert(w, i as i32);
    }
    println!("{:?}", t.starts_with("de")); // ["deer", "deal"]
}
