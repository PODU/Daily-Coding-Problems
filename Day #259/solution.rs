// Day 259: Ghost word game. Build a trie of the dictionary. A starting letter is
// guaranteed safe for player 1 iff every word in its subtree has even length, so
// the opponent is always the one forced to complete a word. Trie DFS, O(total chars).
use std::collections::BTreeMap;

struct T {
    kids: BTreeMap<char, T>,
    word: bool,
}

impl T {
    fn new() -> T { T { kids: BTreeMap::new(), word: false } }
}

fn insert(root: &mut T, w: &str) {
    let mut n = root;
    for c in w.chars() {
        n = n.kids.entry(c).or_insert_with(T::new);
    }
    n.word = true;
}

fn all_even(n: &T, depth: usize) -> bool {
    if n.word && depth % 2 != 0 {
        return false;
    }
    for ch in n.kids.values() {
        if !all_even(ch, depth + 1) {
            return false;
        }
    }
    true
}

fn main() {
    let words = ["cat", "calf", "dog", "bear"];
    let mut root = T::new();
    for w in &words {
        insert(&mut root, w);
    }

    let mut res = String::new(); // BTreeMap -> sorted order
    for (c, ch) in &root.kids {
        if all_even(ch, 1) {
            res.push(*c);
        }
    }
    println!("{}", res);
}
