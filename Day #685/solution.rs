// Reverse words between delimiters while keeping delimiters fixed in position.
// Tokenize (words = maximal non-delim runs incl. interior empties), reverse word list, reassemble. O(n) time/space.
use std::collections::HashSet;

fn reverse_words(s: &str, delims: &HashSet<char>) -> String {
    let mut tokens: Vec<(bool, String)> = Vec::new();
    let mut buf = String::new();
    for c in s.chars() {
        if delims.contains(&c) {
            tokens.push((false, std::mem::take(&mut buf)));
            tokens.push((true, c.to_string()));
        } else {
            buf.push(c);
        }
    }
    if !buf.is_empty() {
        tokens.push((false, buf));
    }

    let mut words: Vec<String> = tokens.iter().filter(|t| !t.0).map(|t| t.1.clone()).collect();
    words.reverse();

    let mut out = String::new();
    let mut wi = 0;
    for t in &tokens {
        if t.0 {
            out.push_str(&t.1);
        } else {
            out.push_str(&words[wi]);
            wi += 1;
        }
    }
    out
}

fn main() {
    let d: HashSet<char> = ['/', ':'].iter().cloned().collect();
    for t in ["hello/world:here", "hello/world:here/", "hello//world:here"] {
        println!("{} -> {}", t, reverse_words(t, &d));
    }
}
