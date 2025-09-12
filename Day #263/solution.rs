// Day 263: Sentence checker over a stream of characters.
// Approach: validate each candidate sentence with a hand-written state machine
// enforcing all four rules (no regex deps). Time O(n) per sentence.

// Rules:
// 1. Starts with a capital letter followed by a lowercase letter or a space.
// 2. Other chars: lowercase letters, separators (, ; :) or terminal marks (. ? ! ‽).
// 3. A single space between each word.
// 4. Ends with a terminal mark immediately following a word.
fn is_valid(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 || !chars[0].is_ascii_uppercase() {
        return false;
    }
    // Rule 1: char after the capital must be lowercase or space.
    if n >= 2 {
        let c = chars[1];
        if !(c.is_ascii_lowercase() || c == ' ') {
            return false;
        }
    }

    let is_sep = |c: char| c == ',' || c == ';' || c == ':';
    let is_term = |c: char| c == '.' || c == '?' || c == '!' || c == '\u{203D}';

    // Track that a terminal mark immediately follows a word letter (rule 4),
    // single spaces between words (rule 3), no leading separator after space.
    let mut prev_was_letter = chars[0].is_ascii_lowercase() || chars[0].is_ascii_uppercase();
    let mut i = 1;
    while i < n {
        let c = chars[i];
        if c.is_ascii_lowercase() {
            prev_was_letter = true;
        } else if is_sep(c) {
            if !prev_was_letter {
                return false; // separator must follow a letter
            }
            prev_was_letter = false;
        } else if c == ' ' {
            if !prev_was_letter && !is_sep(chars[i - 1]) {
                return false; // space must follow a word (or its separator)
            }
            // next char must start a new word (lowercase letter)
            if i + 1 >= n || !chars[i + 1].is_ascii_lowercase() {
                return false; // double space, or space before non-word
            }
            prev_was_letter = false;
        } else if is_term(c) {
            // Rule 4: must be the last char and immediately follow a word letter.
            return i == n - 1 && prev_was_letter;
        } else {
            return false; // illegal character
        }
        i += 1;
    }
    false // no terminal mark found
}

fn main() {
    let tests = [
        "Hello world.",
        "hello world",
        "Hello  world.",
        "The quick, brown fox jumps.",
        "Is this valid?",
        "No terminal mark",
        "Bad ,spacing.",
    ];
    for t in &tests {
        if is_valid(t) {
            println!("{}", t);
        }
    }
}
