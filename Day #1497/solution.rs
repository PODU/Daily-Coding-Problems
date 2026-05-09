// Day 1497: Step words. Dict word w is a step word of s if len(w)==len(s)+1
// and multiset(s) subset of multiset(w). Char-count comparison.
// Time O(D*L), Space O(1) (26-letter alphabet).
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
            let diff = cnt[i] - base[i];
            if diff < 0 {
                ok = false;
                break;
            }
            extra += diff;
        }
        if ok && extra == 1 {
            res.push(w.to_string());
        }
    }
    res
}

fn main() {
    let input = "APPLE";
    let dict = ["APPEAL", "APPLE", "BANANA", "PLEASE", "APPEALS"];
    let res = step_words(input, &dict);
    let quoted: Vec<String> = res.iter().map(|s| format!("\"{}\"", s)).collect();
    println!("[{}]", quoted.join(", "));
}
