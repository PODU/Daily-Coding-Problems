// Day 599: Ghost game - find starting letters that guarantee player 1 a win.
// Approach: build a trie, minimax over it (landing on a word loses). Time O(total chars), Space O(trie).
use std::collections::BTreeMap;

struct Trie {
    ch: BTreeMap<char, Trie>,
    is_word: bool,
}

impl Trie {
    fn new() -> Trie {
        Trie { ch: BTreeMap::new(), is_word: false }
    }
}

fn insert(root: &mut Trie, w: &str) {
    let mut node = root;
    for c in w.chars() {
        node = node.ch.entry(c).or_insert_with(Trie::new);
    }
    node.is_word = true;
}

// True if the player to move from `node` can force the opponent to lose.
fn mover_wins(node: &Trie) -> bool {
    for child in node.ch.values() {
        if child.is_word {
            continue;
        }
        if !mover_wins(child) {
            return true;
        }
    }
    false
}

fn main() {
    let dict = ["cat", "calf", "dog", "bear"];
    let mut root = Trie::new();
    for w in dict.iter() {
        insert(&mut root, w);
    }

    let mut winning: Vec<String> = Vec::new();
    for (c, child) in root.ch.iter() {
        if !child.is_word && !mover_wins(child) {
            winning.push(c.to_string());
        }
    }
    println!("{}", winning.join(" "));
}
