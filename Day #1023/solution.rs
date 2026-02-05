// Day 1023: Alien dictionary - order of letters from sorted words.
// Approach: build precedence graph from adjacent words, Kahn's topological sort.
// Time O(total chars + V + E), Space O(V + E).
use std::collections::{BTreeSet, HashMap, VecDeque};

fn alien_order(words: &[&str]) -> Vec<char> {
    let mut appear: Vec<char> = Vec::new();
    let mut adj: HashMap<char, BTreeSet<char>> = HashMap::new();
    let mut indeg: HashMap<char, i32> = HashMap::new();
    for w in words {
        for c in w.chars() {
            indeg.entry(c).or_insert_with(|| {
                appear.push(c);
                adj.insert(c, BTreeSet::new());
                0
            });
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

    let mut q: VecDeque<char> = appear.iter().cloned().filter(|c| indeg[c] == 0).collect();
    let mut res = Vec::new();
    while let Some(c) = q.pop_front() {
        res.push(c);
        let nbs: Vec<char> = adj[&c].iter().cloned().collect();
        for nb in nbs {
            let e = indeg.get_mut(&nb).unwrap();
            *e -= 1;
            if *e == 0 {
                q.push_back(nb);
            }
        }
    }
    res
}

fn main() {
    let words = ["xww", "wxyz", "wxyw", "ywx", "ywz"];
    let parts: Vec<String> = alien_order(&words).iter().map(|c| format!("'{}'", c)).collect();
    println!("[{}]", parts.join(", "));
}
