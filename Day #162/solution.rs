// Day 162: Shortest unique prefix via trie. Each node stores pass-through count;
// the shortest prefix with count 1 is unique. Time O(total chars), Space O(same).
use std::collections::HashMap;

#[derive(Default)]
struct Node {
    count: u32,
    child: HashMap<char, usize>, // index into arena
}

struct Trie {
    arena: Vec<Node>,
}

impl Trie {
    fn new() -> Self {
        Trie { arena: vec![Node::default()] } // root = index 0
    }
    fn insert(&mut self, word: &str) {
        let mut cur = 0;
        for c in word.chars() {
            let next = match self.arena[cur].child.get(&c) {
                Some(&idx) => idx,
                None => {
                    let idx = self.arena.len();
                    self.arena.push(Node::default());
                    self.arena[cur].child.insert(c, idx);
                    idx
                }
            };
            cur = next;
            self.arena[cur].count += 1;
        }
    }
    fn unique_prefix(&self, word: &str) -> String {
        let mut cur = 0;
        let mut prefix = String::new();
        for c in word.chars() {
            cur = self.arena[cur].child[&c];
            prefix.push(c);
            if self.arena[cur].count == 1 {
                break;
            }
        }
        prefix
    }
}

fn main() {
    let words = ["dog", "cat", "apple", "apricot", "fish"];
    let mut trie = Trie::new();
    for w in &words {
        trie.insert(w);
    }
    for w in &words {
        println!("{}", trie.unique_prefix(w));
    }
}
