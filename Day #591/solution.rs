// Word sense disambiguation via simplified Lesk.
// Score each candidate gloss by content-word overlap with the sentence
// context (other words + their glosses); pick the highest.
// Time O(words * meanings * glossLen). Space O(vocab).
use std::collections::HashSet;

fn stop() -> HashSet<&'static str> {
    [
        "i", "to", "the", "a", "an", "of", "by", "and", "or", "where", "people",
        "that", "is", "are", "in", "on", "at", "with", "went", "sat", "this",
    ]
    .iter()
    .copied()
    .collect()
}

fn tokens(text: &str, sw: &HashSet<&str>) -> HashSet<String> {
    let mut out = HashSet::new();
    for w in text.to_lowercase().split_whitespace() {
        let lw: String = w.chars().filter(|c| c.is_ascii_alphabetic()).collect();
        if !lw.is_empty() && !sw.contains(lw.as_str()) {
            out.insert(lw);
        }
    }
    out
}

fn meaning_for<'a>(word: &str) -> Vec<&'a str> {
    match word {
        "bank" => vec![
            "a financial institution where people deposit and withdraw money",
            "the land alongside a river or lake",
        ],
        "money" => vec!["currency that people deposit and withdraw"],
        "river" => vec!["a large natural stream of water"],
        _ => vec![],
    }
}

fn main() {
    let sw = stop();
    let sentences = [
        "I went to the bank to deposit money",
        "I sat by the bank of the river",
    ];

    for sentence in sentences.iter() {
        let words: Vec<&str> = sentence.split_whitespace().collect();
        for w in &words {
            let lw = w.to_lowercase();
            let cands = meaning_for(&lw);
            if cands.len() <= 1 {
                continue;
            }

            let mut context: HashSet<String> = HashSet::new();
            for other in &words {
                let ol = other.to_lowercase();
                if ol == lw {
                    continue;
                }
                for t in tokens(other, &sw) {
                    context.insert(t);
                }
                for g in meaning_for(&ol) {
                    for t in tokens(g, &sw) {
                        context.insert(t);
                    }
                }
            }

            let mut best: i32 = -1;
            let mut best_gloss = "";
            for gloss in &cands {
                let overlap = tokens(gloss, &sw)
                    .iter()
                    .filter(|t| context.contains(*t))
                    .count() as i32;
                if overlap > best {
                    best = overlap;
                    best_gloss = gloss;
                }
            }
            println!("{}: {}", lw, best_gloss);
        }
    }
}
