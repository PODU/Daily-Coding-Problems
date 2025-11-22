// Day 642: Step words (add one letter + anagram).
// Approach: a dict word qualifies if len == len(word)+1 and its letter counts
// minus the input's are all >= 0 with exactly one extra letter.
// Time: O(D * L), Space: O(1) (26-letter counts).
fn is_step(word: &str, cand: &str) -> bool {
    if cand.len() != word.len() + 1 {
        return false;
    }
    let mut cnt = [0i32; 26];
    for c in cand.bytes() {
        cnt[(c - b'A') as usize] += 1;
    }
    for c in word.bytes() {
        let i = (c - b'A') as usize;
        cnt[i] -= 1;
        if cnt[i] < 0 {
            return false;
        }
    }
    cnt.iter().sum::<i32>() == 1
}

fn step_words<'a>(word: &str, dict: &[&'a str]) -> Vec<&'a str> {
    dict.iter().filter(|w| is_step(word, w)).copied().collect()
}

fn main() {
    let word = "APPLE";
    let dict = vec!["APPEAL", "APPEAR", "PEAR", "APPLES", "PALE"];
    println!("{:?}", step_words(word, &dict)); // ["APPEAL", "APPLES"]
}
