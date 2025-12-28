// Shortest unique prefix via trie of char frequency counts.
// Walk each word until count==1. Time O(total chars), Space O(total chars).
use std::collections::HashMap;

struct Node {
    cnt: u32,
    next: HashMap<char, usize>,
}

struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    fn new() -> Self {
        Trie { nodes: vec![Node { cnt: 0, next: HashMap::new() }] }
    }
    fn insert(&mut self, w: &str) {
        let mut cur = 0;
        for c in w.chars() {
            let nxt = match self.nodes[cur].next.get(&c) {
                Some(&i) => i,
                None => {
                    let i = self.nodes.len();
                    self.nodes.push(Node { cnt: 0, next: HashMap::new() });
                    self.nodes[cur].next.insert(c, i);
                    i
                }
            };
            cur = nxt;
            self.nodes[cur].cnt += 1;
        }
    }
    fn shortest_prefix(&self, w: &str) -> String {
        let mut cur = 0;
        let mut pre = String::new();
        for c in w.chars() {
            cur = self.nodes[cur].next[&c];
            pre.push(c);
            if self.nodes[cur].cnt == 1 {
                break;
            }
        }
        pre
    }
}

fn main() {
    let words = ["dog", "cat", "apple", "apricot", "fish"];
    let mut trie = Trie::new();
    for w in &words {
        trie.insert(w);
    }
    let res: Vec<String> = words.iter().map(|w| trie.shortest_prefix(w)).collect();
    println!("[{}]", res.join(", "));
}
