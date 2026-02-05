// Day 1026: Full text justification.
// Greedy line packing; distribute extra spaces evenly, leftover from the left.
// Time O(total chars), Space O(total chars).
fn justify(words: &[&str], k: usize) -> Vec<String> {
    let mut res = Vec::new();
    let n = words.len();
    let mut i = 0;
    while i < n {
        let mut j = i;
        let mut line_len = words[i].len();
        while j + 1 < n && line_len + 1 + words[j + 1].len() <= k {
            line_len += 1 + words[j + 1].len();
            j += 1;
        }
        let cnt = j - i + 1;
        if cnt == 1 {
            res.push(format!("{}{}", words[i], " ".repeat(k - words[i].len())));
        } else {
            let total_chars: usize = (i..=j).map(|w| words[w].len()).sum();
            let spaces = k - total_chars;
            let gaps = cnt - 1;
            let base = spaces / gaps;
            let extra = spaces % gaps;
            let mut line = String::new();
            for w in i..=j {
                line.push_str(words[w]);
                if w < j {
                    let s = base + if w - i < extra { 1 } else { 0 };
                    line.push_str(&" ".repeat(s));
                }
            }
            res.push(line);
        }
        i = j + 1;
    }
    res
}

fn main() {
    let words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];
    for line in justify(&words, 16) {
        println!("{}", line);
    }
}
