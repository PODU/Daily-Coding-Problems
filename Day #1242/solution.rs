// Word ladder: BFS over words differing by one letter. Time O(N*L*26).
use std::collections::{HashSet, VecDeque};

fn word_ladder(start: &str, end: &str, dictionary: &[&str]) -> Option<Vec<String>> {
    let dict: HashSet<String> = dictionary.iter().map(|s| s.to_string()).collect();
    if !dict.contains(end) {
        return None;
    }
    let mut queue: VecDeque<Vec<String>> = VecDeque::new();
    queue.push_back(vec![start.to_string()]);
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(start.to_string());
    while let Some(path) = queue.pop_front() {
        let word = path.last().unwrap().clone();
        if word == end {
            return Some(path);
        }
        let mut chars: Vec<char> = word.chars().collect();
        for i in 0..chars.len() {
            let orig = chars[i];
            for c in b'a'..=b'z' {
                chars[i] = c as char;
                let nxt: String = chars.iter().collect();
                if dict.contains(&nxt) && !seen.contains(&nxt) {
                    seen.insert(nxt.clone());
                    let mut np = path.clone();
                    np.push(nxt);
                    queue.push_back(np);
                }
            }
            chars[i] = orig;
        }
    }
    None
}

fn main() {
    match word_ladder("dog", "cat", &["dot", "dop", "dat", "cat"]) {
        Some(p) => println!("{:?}", p),
        None => println!("null"),
    }
    match word_ladder("dog", "cat", &["dot", "tod", "dat", "dar"]) {
        Some(p) => println!("{:?}", p),
        None => println!("null"),
    }
}
