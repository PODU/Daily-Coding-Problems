// Day 1623: Sentence equivalence via synonym set.
// Build symmetric synonym set; compare word-by-word. Time O(W), Space O(S).
use std::collections::HashSet;

fn equivalent(a: &str, b: &str, syns: &[(&str, &str)]) -> bool {
    let mut pairs: HashSet<(String, String)> = HashSet::new();
    for &(x, y) in syns {
        pairs.insert((x.to_string(), y.to_string()));
        pairs.insert((y.to_string(), x.to_string()));
    }
    let wa: Vec<&str> = a.split_whitespace().collect();
    let wb: Vec<&str> = b.split_whitespace().collect();
    if wa.len() != wb.len() {
        return false;
    }
    wa.iter().zip(wb.iter()).all(|(&x, &y)| {
        x == y || pairs.contains(&(x.to_string(), y.to_string()))
    })
}

fn main() {
    let syns = [("big", "large"), ("eat", "consume")];
    let eq = equivalent("He wants to eat food.", "He wants to consume food.", &syns);
    println!("{}", if eq { "The two sentences are equivalent." } else { "The two sentences are not equivalent." });
}
