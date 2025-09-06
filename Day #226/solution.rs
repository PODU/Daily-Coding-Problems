// Alien Dictionary: build precedence graph from adjacent words, topological sort (Kahn's BFS).
// Time: O(total characters), Space: O(unique letters + edges).
use std::collections::{BTreeMap, BTreeSet, VecDeque};

fn alien_order(words: &[&str]) -> Vec<char> {
    let mut adj: BTreeMap<char, BTreeSet<char>> = BTreeMap::new();
    let mut indeg: BTreeMap<char, i32> = BTreeMap::new();
    for w in words {
        for c in w.chars() {
            adj.entry(c).or_default();
            indeg.entry(c).or_insert(0);
        }
    }
    for i in 0..words.len().saturating_sub(1) {
        let a: Vec<char> = words[i].chars().collect();
        let b: Vec<char> = words[i + 1].chars().collect();
        let n = a.len().min(b.len());
        for j in 0..n {
            if a[j] != b[j] {
                if adj.get_mut(&a[j]).unwrap().insert(b[j]) {
                    *indeg.get_mut(&b[j]).unwrap() += 1;
                }
                break;
            }
        }
    }
    let mut q: VecDeque<char> = indeg.iter().filter(|(_, &d)| d == 0).map(|(&c, _)| c).collect();
    let mut order = vec![];
    while let Some(c) = q.pop_front() {
        order.push(c);
        let nxs: Vec<char> = adj.get(&c).unwrap().iter().cloned().collect();
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
    let s: Vec<String> = o.iter().map(|c| format!("'{}'", c)).collect();
    println!("[{}]", s.join(", "));
}
