// Day 1122 - Ghost: winning starting letters for player 1 under optimal play
// Trie + minimax over prefixes. A mover loses if every letter completes a word
// or leads to a winning position for the opponent. Time: O(total letters).
use std::collections::BTreeMap;

#[derive(Default)]
struct TrieNode {
    children: BTreeMap<char, TrieNode>,
    is_word: bool,
}

fn insert(root: &mut TrieNode, w: &str) {
    let mut node = root;
    for ch in w.chars() {
        node = node.children.entry(ch).or_default();
    }
    node.is_word = true;
}

fn can_win(node: &TrieNode) -> bool {
    for child in node.children.values() {
        if child.is_word {
            continue;
        }
        if !can_win(child) {
            return true;
        }
    }
    false
}

fn main() {
    let words = ["cat", "calf", "dog", "bear"];
    let mut root = TrieNode::default();
    for w in &words {
        insert(&mut root, w);
    }

    let mut res: Vec<char> = Vec::new();
    for (&ch, child) in &root.children {
        if !child.is_word && !can_win(child) {
            res.push(ch);
        }
    }
    res.sort();
    let s: Vec<String> = res.iter().map(|c| c.to_string()).collect();
    println!("Winning starting letters: {}", s.join(" "));
}
