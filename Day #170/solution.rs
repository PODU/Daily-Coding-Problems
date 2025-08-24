// Word ladder via BFS over one-letter transformations. O(N * L * 26) time, O(N) space.
use std::collections::{HashMap, HashSet, VecDeque};

fn ladder(start: &str, end: &str, dictionary: &[&str]) -> Option<Vec<String>> {
    if start == end {
        return Some(vec![start.to_string()]);
    }
    let dict: HashSet<&str> = dictionary.iter().cloned().collect();
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(start.to_string());
    let mut parent: HashMap<String, Option<String>> = HashMap::new();
    parent.insert(start.to_string(), None);
    while let Some(cur) = queue.pop_front() {
        let bytes: Vec<u8> = cur.bytes().collect();
        for i in 0..bytes.len() {
            let orig = bytes[i];
            for c in b'a'..=b'z' {
                if c == orig {
                    continue;
                }
                let mut nb = bytes.clone();
                nb[i] = c;
                let nxt = String::from_utf8(nb).unwrap();
                let is_target = nxt == end;
                if dict.contains(nxt.as_str()) && !parent.contains_key(&nxt) {
                    parent.insert(nxt.clone(), Some(cur.clone()));
                    if is_target {
                        let mut path = Vec::new();
                        let mut s = Some(nxt);
                        while let Some(w) = s {
                            path.push(w.clone());
                            s = parent.get(&w).cloned().flatten();
                        }
                        path.reverse();
                        return Some(path);
                    }
                    queue.push_back(nxt);
                }
            }
        }
    }
    None
}

fn format(path: Option<Vec<String>>) -> String {
    match path {
        None => "null".to_string(),
        Some(p) => {
            let quoted: Vec<String> = p.iter().map(|w| format!("\"{}\"", w)).collect();
            format!("[{}]", quoted.join(", "))
        }
    }
}

fn main() {
    println!("{}", format(ladder("dog", "cat", &["dot", "dop", "dat", "cat"])));
    println!("{}", format(ladder("dog", "cat", &["dot", "tod", "dat", "dar"])));
}
