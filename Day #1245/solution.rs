// Full text justification: greedy packing + even space distribution (extras
// to the left). Time O(total chars), Space O(output).
fn justify(words: &[&str], k: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let n = words.len();
    let mut i = 0;
    while i < n {
        let mut j = i;
        let mut length = 0;
        while j < n && length + words[j].len() + (j - i) <= k {
            length += words[j].len();
            j += 1;
        }
        let gaps = j - i - 1;
        let line = if gaps == 0 {
            format!("{}{}", words[i], " ".repeat(k - words[i].len()))
        } else {
            let spaces = k - length;
            let base = spaces / gaps;
            let extra = spaces % gaps;
            let mut s = String::new();
            for w in i..j - 1 {
                s.push_str(words[w]);
                let sp = base + if (w - i) < extra { 1 } else { 0 };
                s.push_str(&" ".repeat(sp));
            }
            s.push_str(words[j - 1]);
            s
        };
        lines.push(line);
        i = j;
    }
    lines
}

fn main() {
    let words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];
    for line in justify(&words, 16) {
        println!("\"{}\"", line);
    }
}
