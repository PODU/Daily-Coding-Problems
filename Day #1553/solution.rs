// Alien dictionary: build edges from first differing chars of adjacent words,
// then Kahn's topological sort. Time O(total chars), Space O(letters + edges).
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::cmp::Reverse;

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
        let mut j = 0;
        while j < n {
            if a[j] != b[j] {
                if adj.get_mut(&a[j]).unwrap().insert(b[j]) {
                    *indeg.get_mut(&b[j]).unwrap() += 1;
                }
                break;
            }
            j += 1;
        }
        if j == n && a.len() > b.len() {
            return Vec::new(); // invalid prefix
        }
    }
    let mut heap: BinaryHeap<Reverse<char>> = BinaryHeap::new();
    for (&c, &d) in &indeg {
        if d == 0 {
            heap.push(Reverse(c));
        }
    }
    let mut order = Vec::new();
    while let Some(Reverse(c)) = heap.pop() {
        order.push(c);
        let nxts: Vec<char> = adj[&c].iter().cloned().collect();
        for nx in nxts {
            let e = indeg.get_mut(&nx).unwrap();
            *e -= 1;
            if *e == 0 {
                heap.push(Reverse(nx));
            }
        }
    }
    if order.len() != indeg.len() {
        return Vec::new();
    }
    order
}

fn main() {
    let words = ["xww", "wxyz", "wxyw", "ywx", "ywz"];
    let order = alien_order(&words);
    let parts: Vec<String> = order.iter().map(|c| format!("'{}'", c)).collect();
    println!("[{}]", parts.join(", "));
}
