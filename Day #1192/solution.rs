// Step word: dict word of length len(input)+1 that contains all input letters plus exactly one extra,
// AND is a genuine anagram (rearrangement), not the input with a letter merely appended (no input prefix).
// Compare 26-letter frequency counts: every count >= input's and total diff == 1. O(D * (L + 26)).

fn counts(s: &str) -> [i32; 26] {
    let mut c = [0i32; 26];
    for b in s.bytes() {
        c[(b - b'A') as usize] += 1;
    }
    c
}

fn step_words(word: &str, dict: &[&str]) -> Vec<String> {
    let base = counts(word);
    let mut res = Vec::new();
    for &w in dict {
        if w.len() != word.len() + 1 {
            continue;
        }
        let cnt = counts(w);
        let mut ok = true;
        let mut diff = 0;
        for i in 0..26 {
            if cnt[i] < base[i] {
                ok = false;
                break;
            }
            diff += cnt[i] - base[i];
        }
        if ok && diff == 1 && !w.starts_with(word) {
            res.push(w.to_string());
        }
    }
    res
}

fn main() {
    let word = "APPLE";
    let dict = ["APPEAL", "APPLE", "PEAR", "PALE", "APPEALS", "PAPER", "APPLES", "LAPEL"];
    for w in step_words(word, &dict) {
        println!("{}", w);
    }
}
