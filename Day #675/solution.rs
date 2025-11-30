// Day 675: Sentence equivalence under (non-transitive) synonym pairs. For each position,
// words must be equal or a known synonym pair. Time O(W) with a hashed pair set.
use std::collections::HashSet;

fn tokens(s: &str) -> Vec<String> {
    s.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_string())
        .collect()
}

fn equivalent(synonyms: &[(&str, &str)], s1: &str, s2: &str) -> bool {
    let mut pairs: HashSet<(String, String)> = HashSet::new();
    for (a, b) in synonyms {
        pairs.insert((a.to_string(), b.to_string()));
        pairs.insert((b.to_string(), a.to_string()));
    }
    let (w1, w2) = (tokens(s1), tokens(s2));
    if w1.len() != w2.len() {
        return false;
    }
    w1.iter()
        .zip(w2.iter())
        .all(|(a, b)| a == b || pairs.contains(&(a.clone(), b.clone())))
}

fn main() {
    let syn = [("big", "large"), ("eat", "consume")];
    let ok = equivalent(&syn, "He wants to eat food.", "He wants to consume food.");
    println!("{}", if ok { "True" } else { "False" }); // True
}
