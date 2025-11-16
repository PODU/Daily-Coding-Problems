// Word Ladder: BFS over words, mutating one letter at a time; track parents to rebuild path.
// Time: O(N * L * 26). Space: O(N * L).
use std::collections::{HashMap, HashSet, VecDeque};

fn word_ladder(start: &str, end: &str, dictionary: &[&str]) -> Option<Vec<String>> {
    let dict: HashSet<String> = dictionary.iter().map(|s| s.to_string()).collect();
    if !dict.contains(end) {
        return None;
    }
    let mut parent: HashMap<String, Option<String>> = HashMap::new();
    parent.insert(start.to_string(), None);
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(start.to_string());
    while let Some(cur) = queue.pop_front() {
        if cur == end {
            let mut path = Vec::new();
            let mut w = Some(end.to_string());
            while let Some(word) = w {
                path.push(word.clone());
                w = parent[&word].clone();
            }
            path.reverse();
            return Some(path);
        }
        let chars: Vec<char> = cur.chars().collect();
        for i in 0..chars.len() {
            for c in b'a'..=b'z' {
                let ch = c as char;
                if ch == chars[i] {
                    continue;
                }
                let mut next: Vec<char> = chars.clone();
                next[i] = ch;
                let nxt: String = next.into_iter().collect();
                if dict.contains(&nxt) && !parent.contains_key(&nxt) {
                    parent.insert(nxt.clone(), Some(cur.clone()));
                    queue.push_back(nxt);
                }
            }
        }
    }
    None
}

fn fmt(path: &Option<Vec<String>>) -> String {
    match path {
        None => "null".to_string(),
        Some(p) => {
            let parts: Vec<String> = p.iter().map(|w| format!("\"{}\"", w)).collect();
            format!("[{}]", parts.join(", "))
        }
    }
}

fn main() {
    println!("{}", fmt(&word_ladder("dog", "cat", &["dot", "dop", "dat", "cat"])));
    println!("{}", fmt(&word_ladder("dog", "cat", &["dot", "tod", "dat", "dar"])));
}
