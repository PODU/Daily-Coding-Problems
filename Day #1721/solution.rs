// Reverse words while keeping delimiters in place: split into word/delimiter tokens,
// reverse only the word list, re-emit in original token order. Time O(n), Space O(n).
use std::collections::HashSet;

fn reverse_words(s: &str, delims: &HashSet<char>) -> String {
    let mut tokens: Vec<(String, bool)> = Vec::new(); // (text, is_word)
    let mut cur = String::new();
    for c in s.chars() {
        if delims.contains(&c) {
            if !cur.is_empty() {
                tokens.push((cur.clone(), true));
                cur.clear();
            }
            tokens.push((c.to_string(), false));
        } else {
            cur.push(c);
        }
    }
    if !cur.is_empty() {
        tokens.push((cur.clone(), true));
    }

    let mut words: Vec<String> = tokens.iter().filter(|t| t.1).map(|t| t.0.clone()).collect();
    words.reverse();

    let mut res = String::new();
    let mut wi = 0;
    for t in &tokens {
        if t.1 {
            res.push_str(&words[wi]);
            wi += 1;
        } else {
            res.push_str(&t.0);
        }
    }
    res
}

fn main() {
    let delims: HashSet<char> = ['/', ':'].iter().cloned().collect();
    println!("{}", reverse_words("hello/world:here", &delims));
    println!("{}", reverse_words("hello/world:here/", &delims));
    println!("{}", reverse_words("hello//world:here", &delims));
}
