// Day 114: Reverse words but keep delimiters fixed. Collect words, reverse list,
// re-emit walking original structure. O(n) time, O(n) space.
use std::collections::HashSet;

fn reverse_keep_delims(s: &str, delim: &HashSet<char>) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut words: Vec<String> = Vec::new();
    let mut cur = String::new();
    for &c in &chars {
        if delim.contains(&c) {
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
    let (mut wi, mut i, n) = (0usize, 0usize, chars.len());
    while i < n {
        if delim.contains(&chars[i]) {
            out.push(chars[i]);
            i += 1;
        } else {
            out.push_str(&words[wi]);
            wi += 1;
            while i < n && !delim.contains(&chars[i]) {
                i += 1;
            }
        }
    }
    out
}

fn main() {
    let d: HashSet<char> = ['/', ':'].iter().cloned().collect();
    println!("{}", reverse_keep_delims("hello/world:here", &d));  // here/world:hello
    println!("{}", reverse_keep_delims("hello/world:here/", &d)); // here/world:hello/
    println!("{}", reverse_keep_delims("hello//world:here", &d)); // here//world:hello
}
