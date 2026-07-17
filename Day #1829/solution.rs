// Ghost: build a trie, solve the game bottom-up. A mover wins if some child is
// not a word and is a losing position for the opponent. O(total chars).
use std::collections::BTreeMap;

struct Trie {
    ch: BTreeMap<char, Trie>,
    word: bool,
}

impl Trie {
    fn new() -> Trie {
        Trie { ch: BTreeMap::new(), word: false }
    }
}

fn insert(root: &mut Trie, w: &str) {
    let mut cur = root;
    for c in w.chars() {
        cur = cur.ch.entry(c).or_insert_with(Trie::new);
    }
    cur.word = true;
}

fn can_win(node: &Trie) -> bool {
    for child in node.ch.values() {
        if child.word {
            continue;
        }
        if !can_win(child) {
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
    let mut wins: Vec<String> = Vec::new();
    for (c, child) in root.ch.iter() {
        if !child.word && !can_win(child) {
            wins.push(c.to_string());
        }
    }
    println!("{}", wins.join(", "));
}
