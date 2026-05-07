// Day 1481: Reverse words while keeping delimiters in their original positions.
// Tokenize into word-runs and delimiter chars, reverse word tokens, re-emit.
// Handles trailing/consecutive delimiters. Time O(N), Space O(N).
use std::collections::HashSet;

fn reverse_words(s: &str, delims: &HashSet<char>) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut tokens: Vec<(bool, String)> = Vec::new();
    let mut i = 0;
    let n = chars.len();
    while i < n {
        if delims.contains(&chars[i]) {
            tokens.push((false, chars[i].to_string()));
            i += 1;
        } else {
            let j_start = i;
            while i < n && !delims.contains(&chars[i]) {
                i += 1;
            }
            tokens.push((true, chars[j_start..i].iter().collect()));
        }
    }
    let mut words: Vec<String> = tokens.iter().filter(|t| t.0).map(|t| t.1.clone()).collect();
    words.reverse();
    let mut out = String::new();
    let mut k = 0;
    for t in &tokens {
        if t.0 {
            out.push_str(&words[k]);
            k += 1;
        } else {
            out.push_str(&t.1);
        }
    }
    out
}

fn main() {
    let d: HashSet<char> = ['/', ':'].iter().cloned().collect();
    println!("{}", reverse_words("hello/world:here", &d));   // here/world:hello
    println!("{}", reverse_words("hello/world:here/", &d));  // here/world:hello/
    println!("{}", reverse_words("hello//world:here", &d));  // here//world:hello
}
