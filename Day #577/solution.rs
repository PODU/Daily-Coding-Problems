// Word circle: model words as directed edges first->last char; Eulerian circuit via Hierholzer. Time O(V+E), Space O(V+E).
use std::collections::HashMap;

fn word_circle(words: &[&str]) -> Vec<String> {
    let mut adj: HashMap<char, Vec<(char, String)>> = HashMap::new();
    for w in words {
        let f = w.chars().next().unwrap();
        let l = w.chars().last().unwrap();
        adj.entry(f).or_default().push((l, w.to_string()));
    }
    let mut ptr: HashMap<char, usize> = HashMap::new();

    let start = words[0].chars().next().unwrap();
    let mut path: Vec<String> = Vec::new();
    // stack frames: (char, Option<word used to enter>)
    let mut stack: Vec<(char, Option<String>)> = vec![(start, None)];
    while let Some(&(v, _)) = stack.last() {
        let empty: Vec<(char, String)> = Vec::new();
        let lst = adj.get(&v).unwrap_or(&empty);
        let p = *ptr.get(&v).unwrap_or(&0);
        if p < lst.len() {
            ptr.insert(v, p + 1);
            let (nxt, w) = lst[p].clone();
            stack.push((nxt, Some(w)));
        } else {
            let (_, w) = stack.pop().unwrap();
            if let Some(word) = w {
                path.push(word);
            }
        }
    }
    path.reverse();
    path
}

fn main() {
    let words = ["chair", "height", "racket", "touch", "tunic"];
    let path = word_circle(&words);
    println!("{} --> {}", path.join(" --> "), path[0]);
}
