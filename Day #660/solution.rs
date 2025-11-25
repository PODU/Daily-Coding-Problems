// Alien dictionary: build edges from first differing char of adjacent words,
// then Kahn's BFS topological sort. Time O(C + V + E), Space O(V + E).
use std::collections::{BTreeMap, BTreeSet};

fn alien_order(words: &[&str]) -> Vec<char> {
    let mut graph: BTreeMap<char, BTreeSet<char>> = BTreeMap::new();
    let mut indeg: BTreeMap<char, i32> = BTreeMap::new();
    for w in words {
        for c in w.chars() {
            graph.entry(c).or_default();
            indeg.entry(c).or_insert(0);
        }
    }
    for i in 0..words.len().saturating_sub(1) {
        let a: Vec<char> = words[i].chars().collect();
        let b: Vec<char> = words[i + 1].chars().collect();
        let n = a.len().min(b.len());
        for j in 0..n {
            if a[j] != b[j] {
                if graph.get_mut(&a[j]).unwrap().insert(b[j]) {
                    *indeg.get_mut(&b[j]).unwrap() += 1;
                }
                break;
            }
        }
    }
    let mut queue: BTreeSet<char> =
        indeg.iter().filter(|(_, &d)| d == 0).map(|(&c, _)| c).collect();
    let mut order = Vec::new();
    while let Some(&c) = queue.iter().next() {
        queue.remove(&c);
        order.push(c);
        let nexts: Vec<char> = graph[&c].iter().cloned().collect();
        for nxt in nexts {
            let d = indeg.get_mut(&nxt).unwrap();
            *d -= 1;
            if *d == 0 {
                queue.insert(nxt);
            }
        }
    }
    order
}

fn main() {
    let words = ["xww", "wxyz", "wxyw", "ywx", "ywz"];
    let order = alien_order(&words);
    let parts: Vec<String> = order.iter().map(|c| format!("'{}'", c)).collect();
    println!("[{}]", parts.join(", "));
}
