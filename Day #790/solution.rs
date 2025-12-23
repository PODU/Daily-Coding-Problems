// Step words: a dict word qualifies if len==word.len+1 and word's letter
// multiset is a subset leaving exactly one extra letter. O(dict*wordLen) time, O(1) space.
fn step_words(word: &str, dict: &[&str]) -> Vec<String> {
    let mut base = [0i32; 26];
    for c in word.bytes() {
        base[(c - b'A') as usize] += 1;
    }
    let mut res = Vec::new();
    for &w in dict {
        if w.len() != word.len() + 1 {
            continue;
        }
        let mut cnt = [0i32; 26];
        for c in w.bytes() {
            cnt[(c - b'A') as usize] += 1;
        }
        let mut extra = 0;
        let mut ok = true;
        for i in 0..26 {
            let d = cnt[i] - base[i];
            if d < 0 {
                ok = false;
                break;
            }
            extra += d;
        }
        if ok && extra == 1 {
            res.push(w.to_string());
        }
    }
    res
}

fn main() {
    let word = "APPLE";
    let dict = ["APPEAL", "BANANA", "ORANGE", "GRAPE"];
    println!("{}", step_words(word, &dict).join(" "));
}
