// Simplified Lesk WSD: score each candidate meaning by overlap with the union of other
// in-dict context words and their meaning texts. O(W*M*L) time, O(V) space.
use std::collections::HashSet;

fn words(s: &str) -> Vec<String> {
    s.to_lowercase().split_whitespace().map(|x| x.to_string()).collect()
}

fn main() {
    let meanings: Vec<(&str, Vec<&str>)> = vec![
        ("bank", vec!["place where people deposit and withdraw money", "land beside a river or lake"]),
        ("money", vec!["currency coins and cash used to buy things"]),
        ("river", vec!["a large natural stream of flowing water"]),
    ];
    let lookup = |w: &str| meanings.iter().find(|(k, _)| *k == w).map(|(_, v)| v.clone());
    let sentence = "I went to get money from the bank";
    let toks = words(sentence);
    for w in &toks {
        if let Some(cands) = lookup(w) {
            if cands.len() > 1 {
                let mut ctx: HashSet<String> = HashSet::new();
                for o in &toks {
                    if o != w {
                        if let Some(om) = lookup(o) {
                            ctx.insert(o.clone());
                            for m in &om {
                                for x in words(m) {
                                    ctx.insert(x);
                                }
                            }
                        }
                    }
                }
                let mut best: &str = cands[0];
                let mut best_score = -1i32;
                for cand in &cands {
                    let mut score = 0i32;
                    for t in words(cand) {
                        if ctx.contains(&t) {
                            score += 1;
                        }
                    }
                    if score > best_score {
                        best_score = score;
                        best = *cand;
                    }
                }
                println!("{}: {}", w, best);
            }
        }
    }
}
