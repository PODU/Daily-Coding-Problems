// Day 431: Sentence validator via finite-state-machine scan over chars.
// Mirrors regex ^[A-Z][a-z]*([,;:]? [a-z]+)*[,;:]?[.?!\u{203D}]$ (no backtracking needed).
// O(n) time, O(n) space per sentence.
fn is_sep(c: char) -> bool { c == ',' || c == ';' || c == ':' }
fn is_term(c: char) -> bool { c == '.' || c == '?' || c == '!' || c == '\u{203D}' }
fn is_low(c: char) -> bool { c >= 'a' && c <= 'z' }

fn is_valid_sentence(s: &str) -> bool {
    let a: Vec<char> = s.chars().collect();
    let n = a.len();
    if n == 0 { return false; }
    if !(a[0] >= 'A' && a[0] <= 'Z') { return false; }
    let mut i = 1;
    while i < n && is_low(a[i]) { i += 1; }
    loop {
        let mut j = i;
        if j < n && is_sep(a[j]) { j += 1; }
        if j < n && a[j] == ' ' {
            j += 1;
            if j < n && is_low(a[j]) {
                while j < n && is_low(a[j]) { j += 1; }
                i = j;
                continue;
            }
        }
        break;
    }
    if i < n && is_sep(a[i]) { i += 1; }
    i == n - 1 && is_term(a[i])
}

fn main() {
    let tests = ["The quick brown fox.", "hello world.", "Hello  world.",
                 "Hello world", "Hi there, friend!"];
    for t in tests.iter() {
        println!("{}{}", if is_valid_sentence(t) { "Valid: " } else { "Invalid: " }, t);
    }
}
