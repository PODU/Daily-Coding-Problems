// Reverse words but keep delimiters fixed in place: extract words, reverse the list,
// rebuild keeping delimiter chars at original positions. O(n) time, O(n) space.

fn is_delim(c: char) -> bool {
    c == '/' || c == ':'
}

fn reverse_words(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut words: Vec<String> = Vec::new();
    let mut cur = String::new();
    for &c in &chars {
        if is_delim(c) {
            if !cur.is_empty() {
                words.push(cur.clone());
                cur.clear();
            }
        } else {
            cur.push(c);
        }
    }
    if !cur.is_empty() {
        words.push(cur);
    }
    words.reverse();

    let mut out = String::new();
    let mut wi = 0;
    let mut i = 0;
    let n = chars.len();
    while i < n {
        if is_delim(chars[i]) {
            out.push(chars[i]);
            i += 1;
        } else {
            out.push_str(&words[wi]);
            wi += 1;
            while i < n && !is_delim(chars[i]) {
                i += 1;
            }
        }
    }
    out
}

fn main() {
    let tests = ["hello/world:here", "hello/world:here/", "hello//world:here"];
    for t in tests.iter() {
        println!("{} -> {}", t, reverse_words(t));
    }
}
