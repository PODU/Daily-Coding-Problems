// Full text justification: greedily pack words per line, distribute spaces evenly
// with extra spaces favoring left gaps; last/single word left-justified. Time O(total chars).

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
        let mut line = String::new();
        if gaps == 0 {
            line.push_str(words[i]);
            line.push_str(&" ".repeat(k - words[i].len()));
        } else {
            let mut total_spaces = k;
            for w in i..=j {
                total_spaces -= words[w].len();
            }
            let base = total_spaces / gaps;
            let extra = total_spaces % gaps;
            for w in i..=j {
                line.push_str(words[w]);
                if w < j {
                    let sp = base + if w - i < extra { 1 } else { 0 };
                    line.push_str(&" ".repeat(sp));
                }
            }
        }
        res.push(line);
        i = j + 1;
    }
    res
}

fn main() {
    let words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];
    for line in justify(&words, 16) {
        println!("\"{}\"", line);
    }
}
