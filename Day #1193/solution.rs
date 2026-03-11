// Ghost word game: trie + game theory. can_win(node)=mover can force win.
// Winning start c: child not a word AND opponent (can_win(child)) cannot win.
// Time O(total chars), Space O(total chars).
// NOTE: README sample shows only "b" but "c" is also winning.
use std::collections::BTreeMap;

#[derive(Default)]
struct Node {
    ch: BTreeMap<char, Node>,
    is_word: bool,
}

fn insert(root: &mut Node, w: &str) {
    let mut cur = root;
    for c in w.chars() {
        cur = cur.ch.entry(c).or_default();
    }
    cur.is_word = true;
}

// can the player about to move from this prefix force a win?
fn can_win(node: &Node) -> bool {
    for child in node.ch.values() {
        if child.is_word {
            continue; // completing a word loses
        }
        if !can_win(child) {
            return true; // opponent loses
        }
    }
    false
}

fn main() {
    let dict = ["cat", "calf", "dog", "bear"];
    let mut root = Node::default();
    for w in dict.iter() {
        insert(&mut root, w);
    }

    let mut wins: Vec<String> = Vec::new();
    for (c, child) in root.ch.iter() {
        if !child.is_word && !can_win(child) {
            wins.push(c.to_string());
        }
    }
    wins.sort();
    println!("{}", wins.join(" "));
}
