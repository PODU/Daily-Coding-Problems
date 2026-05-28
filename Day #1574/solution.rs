// Shortest unique prefix via Trie storing char frequency counts. O(total chars) time & space.
use std::collections::HashMap;

#[derive(Default)]
struct Node {
    ch: HashMap<char, usize>,
    cnt: i32,
}

fn main() {
    let words = ["dog", "cat", "apple", "apricot", "fish"];
    let mut nodes: Vec<Node> = vec![Node::default()]; // index 0 = root
    for w in &words {
        let mut cur = 0usize;
        for c in w.chars() {
            let next = match nodes[cur].ch.get(&c) {
                Some(&i) => i,
                None => {
                    let i = nodes.len();
                    nodes.push(Node::default());
                    nodes[cur].ch.insert(c, i);
                    i
                }
            };
            nodes[next].cnt += 1;
            cur = next;
        }
    }
    for w in &words {
        let mut cur = 0usize;
        let mut pre = String::new();
        for c in w.chars() {
            cur = nodes[cur].ch[&c];
            pre.push(c);
            if nodes[cur].cnt == 1 {
                break;
            }
        }
        println!("{}", pre);
    }
}
