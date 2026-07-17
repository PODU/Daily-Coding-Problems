// Word circle = Eulerian circuit in graph where each word is an edge first->last char.
// Check in==out degrees, then Hierholzer to build chain. O(V+E).
use std::collections::HashMap;

fn solve(words: &Vec<String>) -> String {
    let mut adj: HashMap<u8, Vec<(String, u8)>> = HashMap::new();
    let mut indeg: HashMap<u8, i32> = HashMap::new();
    let mut outdeg: HashMap<u8, i32> = HashMap::new();
    let mut idx: HashMap<u8, usize> = HashMap::new();
    for w in words {
        let a = w.as_bytes()[0];
        let b = w.as_bytes()[w.len() - 1];
        adj.entry(a).or_default().push((w.clone(), b));
        *outdeg.entry(a).or_insert(0) += 1;
        *indeg.entry(b).or_insert(0) += 1;
    }
    let mut nodes: Vec<u8> = indeg.keys().chain(outdeg.keys()).cloned().collect();
    nodes.sort();
    nodes.dedup();
    for &c in &nodes {
        if *indeg.get(&c).unwrap_or(&0) != *outdeg.get(&c).unwrap_or(&0) {
            return "No circle".to_string();
        }
    }

    let start = words[0].as_bytes()[0];
    let mut st: Vec<u8> = vec![start];
    let mut edge_stack: Vec<String> = Vec::new();
    let mut circuit: Vec<String> = Vec::new();
    let empty: Vec<(String, u8)> = Vec::new();
    while let Some(&u) = st.last() {
        let lst = adj.get(&u).unwrap_or(&empty);
        let i = *idx.get(&u).unwrap_or(&0);
        if i < lst.len() {
            idx.insert(u, i + 1);
            let (w, v) = lst[i].clone();
            st.push(v);
            edge_stack.push(w);
        } else {
            st.pop();
            if let Some(w) = edge_stack.pop() {
                circuit.push(w);
            }
        }
    }
    if circuit.len() != words.len() {
        return "No circle".to_string();
    }
    circuit.reverse();
    format!("{} --> {}", circuit.join(" --> "), circuit[0])
}

fn main() {
    let words: Vec<String> = ["chair", "height", "racket", "touch", "tunic"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    println!("{}", solve(&words));
}
