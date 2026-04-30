// Day 1443: Word sense disambiguation (simplified Lesk algorithm).
// For each ambiguous word pick the meaning whose words overlap most with the
// rest of the sentence's words. Time O(W * M * L), Space O(vocab).
use std::collections::{HashMap, HashSet};

fn tokenize(s: &str) -> Vec<String> {
    s.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect()
}

fn disambiguate(meanings: &HashMap<&str, Vec<&str>>, sentence: &str) -> HashMap<String, String> {
    let words = tokenize(sentence);
    let context: HashSet<&str> = words.iter().map(|s| s.as_str()).collect();
    let mut result = HashMap::new();
    for w in &words {
        let senses = match meanings.get(w.as_str()) {
            Some(s) if s.len() > 1 => s,
            _ => continue,
        };
        let mut best = -1i32;
        let mut best_meaning = String::new();
        for m in senses {
            let mt: HashSet<String> = tokenize(m).into_iter().collect();
            let mut score = 0;
            for t in &mt {
                if t != w && context.contains(t.as_str()) {
                    score += 1;
                }
            }
            if score > best {
                best = score;
                best_meaning = m.to_string();
            }
        }
        result.insert(w.clone(), best_meaning);
    }
    result
}

fn main() {
    let mut meanings: HashMap<&str, Vec<&str>> = HashMap::new();
    meanings.insert(
        "bank",
        vec![
            "financial institution where people deposit money",
            "sloping land beside a river or lake",
        ],
    );
    let sentence = "I went to the bank to deposit money";
    let res = disambiguate(&meanings, sentence);
    println!("bank: {}", res["bank"]);
}
