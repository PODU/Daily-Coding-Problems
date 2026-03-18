// Trie with pass-through counts; for each word walk down until count==1 to get its shortest unique prefix.
// Time: O(total chars), Space: O(total chars).
use std::collections::HashMap;

struct Trie {
    next: HashMap<char, Trie>,
    count: i32,
}

impl Trie {
    fn new() -> Self {
        Trie { next: HashMap::new(), count: 0 }
    }

    fn insert(&mut self, word: &str) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.next.entry(c).or_insert_with(Trie::new);
            cur.count += 1;
        }
    }

    fn prefix(&self, word: &str) -> String {
        let mut cur = self;
        let mut p = String::new();
        for c in word.chars() {
            cur = cur.next.get(&c).unwrap();
            p.push(c);
            if cur.count == 1 {
                break;
            }
        }
        p
    }
}

fn main() {
    let words = ["dog", "cat", "apple", "apricot", "fish"];
    let mut root = Trie::new();
    for w in &words {
        root.insert(w);
    }
    let out: Vec<String> = words.iter().map(|w| root.prefix(w)).collect();
    println!("[{}]", out.join(", "));
}
