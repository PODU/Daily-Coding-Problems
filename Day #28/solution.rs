// Text justification: greedy line packing, distribute spaces with extras to LEFT gaps.
// Time: O(total chars), Space: O(total chars) for output.
fn justify(words: &[&str], k: usize) -> Vec<String> {
    let mut res = Vec::new();
    let n = words.len();
    let mut i = 0;
    while i < n {
        let mut j = i;
        let mut line_len = words[i].len();
        while j + 1 < n && line_len + 1 + words[j + 1].len() <= k {
            j += 1;
            line_len += 1 + words[j].len();
        }
        let gaps = j - i;
        let line = if gaps == 0 {
            format!("{}{}", words[i], " ".repeat(k - words[i].len()))
        } else {
            let total_chars: usize = (i..=j).map(|w| words[w].len()).sum();
            let total_spaces = k - total_chars;
            let base = total_spaces / gaps;
            let extra = total_spaces % gaps;
            let mut s = String::new();
            for w in i..=j {
                s.push_str(words[w]);
                if w < j {
                    let sp = base + if w - i < extra { 1 } else { 0 };
                    s.push_str(&" ".repeat(sp));
                }
            }
            s
        };
        res.push(line);
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
