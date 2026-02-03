// Word ladder: BFS over words, change one letter per step, track predecessors.
// Time: O(N * L * 26), Space: O(N). Returns shortest path or None (null).
use std::collections::{HashMap, HashSet, VecDeque};

fn ladder(start: &str, end: &str, dict: &[&str]) -> Option<Vec<String>> {
    if start == end {
        return Some(vec![start.to_string()]);
    }
    let words: HashSet<String> = dict.iter().map(|s| s.to_string()).collect();
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(start.to_string());
    let mut parent: HashMap<String, Option<String>> = HashMap::new();
    parent.insert(start.to_string(), None);
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start.to_string());

    while let Some(cur) = queue.pop_front() {
        let chars: Vec<char> = cur.chars().collect();
        for i in 0..chars.len() {
            for c in b'a'..=b'z' {
                let ch = c as char;
                if ch == chars[i] {
                    continue;
                }
                let mut nxt_chars = chars.clone();
                nxt_chars[i] = ch;
                let nxt: String = nxt_chars.into_iter().collect();
                if words.contains(&nxt) && !visited.contains(&nxt) {
                    visited.insert(nxt.clone());
                    parent.insert(nxt.clone(), Some(cur.clone()));
                    if nxt == end {
                        let mut path = Vec::new();
                        let mut w = Some(nxt);
                        while let Some(node) = w {
                            path.push(node.clone());
                            w = parent.get(&node).cloned().flatten();
                        }
                        path.reverse();
                        return Some(path);
                    }
                    queue.push_back(nxt);
                }
            }
        }
    }
    None // no path
}

fn print_path(p: Option<Vec<String>>) {
    match p {
        None => println!("null"),
        Some(v) => {
            let quoted: Vec<String> = v.iter().map(|w| format!("\"{}\"", w)).collect();
            println!("[{}]", quoted.join(", "));
        }
    }
}

fn main() {
    print_path(ladder("dog", "cat", &["dot", "dop", "dat", "cat"])); // ["dog", "dot", "dat", "cat"]
    print_path(ladder("dog", "cat", &["dot", "tod", "dat", "dar"])); // null
}
