// Trie with per-node pass counts; shortest unique prefix = up to first node count==1. Time O(total chars).
use std::collections::HashMap;

#[derive(Default)]
struct Node {
    next: HashMap<char, usize>,
    count: u32,
}

fn shortest_unique_prefixes(words: &[&str]) -> Vec<String> {
    let mut trie: Vec<Node> = vec![Node::default()];
    for w in words {
        let mut cur = 0usize;
        for c in w.chars() {
            let nxt = match trie[cur].next.get(&c) {
                Some(&idx) => idx,
                None => {
                    let idx = trie.len();
                    trie.push(Node::default());
                    trie[cur].next.insert(c, idx);
                    idx
                }
            };
            cur = nxt;
            trie[cur].count += 1;
        }
    }
    let mut res = Vec::with_capacity(words.len());
    for w in words {
        let mut cur = 0usize;
        let mut pre = String::new();
        for c in w.chars() {
            cur = trie[cur].next[&c];
            pre.push(c);
            if trie[cur].count == 1 {
                break;
            }
        }
        res.push(pre);
    }
    res
}

fn main() {
    let words = ["dog", "cat", "apple", "apricot", "fish"];
    println!("{:?}", shortest_unique_prefixes(&words));
}
