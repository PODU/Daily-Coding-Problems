// Day 1203: Alien dictionary - derive letter order from sorted words.
// Build precedence graph from adjacent word diffs, Kahn topological sort. Time O(total chars), Space O(1) alphabet.
use std::collections::{BTreeSet, HashMap, VecDeque};

fn alien_order(words: &[&str]) -> Vec<char> {
    let mut adj: HashMap<char, BTreeSet<char>> = HashMap::new();
    let mut indeg: HashMap<char, i32> = HashMap::new();
    for w in words {
        for c in w.chars() {
            adj.entry(c).or_default();
            indeg.entry(c).or_insert(0);
        }
    }
    for i in 0..words.len().saturating_sub(1) {
        let a: Vec<char> = words[i].chars().collect();
        let b: Vec<char> = words[i + 1].chars().collect();
        for j in 0..a.len().min(b.len()) {
            if a[j] != b[j] {
                if adj.get_mut(&a[j]).unwrap().insert(b[j]) {
                    *indeg.get_mut(&b[j]).unwrap() += 1;
                }
                break;
            }
        }
    }
    let mut keys: Vec<char> = indeg.iter().filter(|(_, &d)| d == 0).map(|(&c, _)| c).collect();
    keys.sort();
    let mut q: VecDeque<char> = keys.into_iter().collect();
    let mut order = Vec::new();
    while let Some(c) = q.pop_front() {
        order.push(c);
        let nxs: Vec<char> = adj[&c].iter().cloned().collect();
        for nx in nxs {
            let e = indeg.get_mut(&nx).unwrap();
            *e -= 1;
            if *e == 0 {
                q.push_back(nx);
            }
        }
    }
    order
}

fn main() {
    let words = ["xww", "wxyz", "wxyw", "ywx", "ywz"];
    let o = alien_order(&words);
    let parts: Vec<String> = o.iter().map(|c| format!("'{}'", c)).collect();
    println!("[{}]", parts.join(", ")); // ['x', 'z', 'w', 'y']
}
