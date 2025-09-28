// Sentence Similarity. Direct (non-transitive) synonym pairs via HashSet.
// Time O(P + N) for P pairs and N words. Space O(P).
// Secondary union-find approach (transitive follow-up) included below.
use std::collections::{HashMap, HashSet};

fn tokenize(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(|w| w.trim_end_matches(['.', ',', '!', '?', ';', ':']).to_string())
        .collect()
}

fn are_similar(synonyms: &[(&str, &str)], s1: &str, s2: &str) -> bool {
    let mut pairs: HashSet<(String, String)> = HashSet::new();
    for &(a, b) in synonyms {
        pairs.insert((a.to_string(), b.to_string()));
        pairs.insert((b.to_string(), a.to_string()));
    }
    let w1 = tokenize(s1);
    let w2 = tokenize(s2);
    if w1.len() != w2.len() {
        return false;
    }
    for (x, y) in w1.iter().zip(w2.iter()) {
        if x == y || pairs.contains(&(x.clone(), y.clone())) {
            continue;
        }
        return false;
    }
    true
}

// --- Follow-up (transitive): union-find ---
fn find(parent: &mut HashMap<String, String>, x: &str) -> String {
    let mut x = x.to_string();
    parent.entry(x.clone()).or_insert_with(|| x.clone());
    while parent[&x] != x {
        let gp = parent[&parent[&x]].clone();
        parent.insert(x.clone(), gp.clone());
        x = gp;
    }
    x
}

fn are_similar_transitive(synonyms: &[(&str, &str)], s1: &str, s2: &str) -> bool {
    let mut parent: HashMap<String, String> = HashMap::new();
    for &(a, b) in synonyms {
        let ra = find(&mut parent, a);
        let rb = find(&mut parent, b);
        parent.insert(ra, rb);
    }
    let w1 = tokenize(s1);
    let w2 = tokenize(s2);
    if w1.len() != w2.len() {
        return false;
    }
    w1.iter()
        .zip(w2.iter())
        .all(|(x, y)| x == y || find(&mut parent, x) == find(&mut parent, y))
}

fn main() {
    let synonyms = [("big", "large"), ("eat", "consume")];
    let s1 = "He wants to eat food.";
    let s2 = "He wants to consume food.";
    println!("{}", if are_similar(&synonyms, s1, s2) { "equivalent" } else { "not equivalent" });
}
