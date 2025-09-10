// Words form a circle: model each word as a directed edge first->last char; an Eulerian circuit
// exists iff in-degree==out-degree for every node and edges form one connected component.
// Find the circuit with Hierholzer's algorithm. O(V + E) time and space.
use std::collections::{HashMap, HashSet};

fn circle_order(words: &[&str]) -> Option<Vec<String>> {
    let mut adj: HashMap<char, Vec<(char, String)>> = HashMap::new();
    let mut indeg: HashMap<char, i32> = HashMap::new();
    let mut outdeg: HashMap<char, i32> = HashMap::new();
    let mut nodes: HashSet<char> = HashSet::new();
    for &w in words {
        let a = w.chars().next().unwrap();
        let b = w.chars().last().unwrap();
        adj.entry(a).or_default().push((b, w.to_string()));
        *outdeg.entry(a).or_insert(0) += 1;
        *indeg.entry(b).or_insert(0) += 1;
        nodes.insert(a);
        nodes.insert(b);
    }
    for &c in &nodes {
        if indeg.get(&c).copied().unwrap_or(0) != outdeg.get(&c).copied().unwrap_or(0) {
            return None;
        }
    }

    // Connectivity (undirected) over nodes with outgoing edges.
    let mut und: HashMap<char, Vec<char>> = HashMap::new();
    for (&a, lst) in &adj {
        for (b, _) in lst {
            und.entry(a).or_default().push(*b);
            und.entry(*b).or_default().push(a);
        }
    }
    let active: Vec<char> = adj.iter().filter(|(_, l)| !l.is_empty()).map(|(&a, _)| a).collect();
    if active.is_empty() {
        return None;
    }
    let mut seen: HashSet<char> = HashSet::new();
    let mut st = vec![active[0]];
    while let Some(c) = st.pop() {
        if !seen.insert(c) {
            continue;
        }
        if let Some(nbs) = und.get(&c) {
            for &nb in nbs {
                if !seen.contains(&nb) {
                    st.push(nb);
                }
            }
        }
    }
    if active.iter().any(|c| !seen.contains(c)) {
        return None;
    }

    // Hierholzer starting from first word's first char.
    let start = words[0].chars().next().unwrap();
    let mut ptr: HashMap<char, usize> = HashMap::new();
    let mut node_stack = vec![start];
    let mut edge_stack: Vec<String> = Vec::new();
    let mut circuit: Vec<String> = Vec::new();
    while let Some(&v) = node_stack.last() {
        let empty = Vec::new();
        let lst = adj.get(&v).unwrap_or(&empty);
        let p = *ptr.get(&v).unwrap_or(&0);
        if p < lst.len() {
            ptr.insert(v, p + 1);
            let (w, word) = lst[p].clone();
            node_stack.push(w);
            edge_stack.push(word);
        } else {
            node_stack.pop();
            if let Some(word) = edge_stack.pop() {
                circuit.push(word);
            }
        }
    }
    if circuit.len() != words.len() {
        return None;
    }
    circuit.reverse();
    Some(circuit)
}

fn main() {
    let words = ["chair", "height", "racket", "touch", "tunic"];
    match circle_order(&words) {
        Some(order) => println!("{} --> {}", order.join(" --> "), order[0]),
        None => println!("No circle possible"),
    }
}
