// Day 266: Step words. A dict word is a step word of `word` if its length is
// one greater and its letter multiset is a superset of `word`'s (diff = 1).
// Time O(D * L) over dictionary; space O(alphabet).

fn is_step_word(word: &str, cand: &str) -> bool {
    if cand.chars().count() != word.chars().count() + 1 {
        return false;
    }
    let mut cnt = [0i32; 26];
    for c in cand.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
    }
    for c in word.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            let i = (c as u8 - b'a') as usize;
            cnt[i] -= 1;
            if cnt[i] < 0 {
                return false;
            }
        }
    }
    cnt.iter().sum::<i32>() == 1
}

fn step_words<'a>(word: &str, dict: &[&'a str]) -> Vec<&'a str> {
    dict.iter().cloned().filter(|w| is_step_word(word, w)).collect()
}

fn main() {
    let word = "APPLE";
    let dict = ["APPEAL", "APPLES", "KELP", "PALE", "APPLE"];
    let res = step_words(word, &dict);
    println!("[{}]", res.join(", "));
}
