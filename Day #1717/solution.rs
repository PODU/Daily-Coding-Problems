// Day 1717: Fully justify text into lines of length k.
// Greedy line packing + even space distribution (extras from left).
// Time: O(total characters), Space: O(output).

fn justify(words: &[&str], k: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let n = words.len();
    let mut i = 0;
    while i < n {
        let mut j = i;
        let mut line_len = words[i].len();
        while j + 1 < n && line_len + 1 + words[j + 1].len() <= k {
            j += 1;
            line_len += 1 + words[j].len();
        }
        let cnt = j - i + 1;
        let word_chars: usize = (i..=j).map(|t| words[t].len()).sum();
        let mut line = String::new();
        if cnt == 1 {
            line.push_str(words[i]);
            line.push_str(&" ".repeat(k - words[i].len()));
        } else {
            let gaps = cnt - 1;
            let total_spaces = k - word_chars;
            let base = total_spaces / gaps;
            let extra = total_spaces % gaps;
            for t in i..=j {
                line.push_str(words[t]);
                if t < j {
                    let sp = base + if t - i < extra { 1 } else { 0 };
                    line.push_str(&" ".repeat(sp));
                }
            }
        }
        lines.push(line);
        i = j + 1;
    }
    lines
}

fn main() {
    let words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];
    for l in justify(&words, 16) {
        println!("\"{}\"", l);
    }
}
