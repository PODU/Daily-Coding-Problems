// Word sense disambiguation: for each ambiguous word pick the meaning whose words
// overlap most with the sentence context (other words). Tie-break -> first meaning.
// Time: O(W * M * L), Space: O(L) for the context set.
use std::collections::HashSet;

fn tokenize(s: &str) -> Vec<String> {
    s.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect()
}

fn main() {
    let dict: Vec<(&str, Vec<&str>)> = vec![(
        "bank",
        vec![
            "financial institution where people deposit money",
            "land beside a river or lake",
        ],
    )];
    let sentence = "I went to get money from the bank";
    let words = tokenize(sentence);

    for (i, w) in words.iter().enumerate() {
        let meanings = match dict.iter().find(|(k, _)| *k == w.as_str()) {
            Some((_, m)) if m.len() > 1 => m,
            _ => continue,
        };
        let context: HashSet<&String> = words
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, x)| x)
            .collect();

        let (mut best, mut best_score) = (0usize, -1i32);
        for (m, meaning) in meanings.iter().enumerate() {
            let score = tokenize(meaning)
                .iter()
                .filter(|t| context.contains(t))
                .count() as i32;
            if score > best_score {
                best_score = score;
                best = m;
            }
        }
        println!("{}: {}", w, meanings[best]);
    }
}
