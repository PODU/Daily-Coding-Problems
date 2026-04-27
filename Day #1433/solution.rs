// Day 1433: Sentence checker over a character stream.
// Approach: finite-state machine validating one sentence at a time. Time: O(n), Space: O(n) buffer.

fn is_terminal(c: char) -> bool {
    c == '.' || c == '?' || c == '!' || c == '\u{203D}' // ‽
}

fn is_separator(c: char) -> bool {
    c == ',' || c == ';' || c == ':'
}

fn is_valid_sentence(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n < 2 {
        return false;
    }
    // Rule 1: capital then lowercase or space.
    if !chars[0].is_uppercase() {
        return false;
    }
    if !(chars[1].is_lowercase() || chars[1] == ' ') {
        return false;
    }

    let mut prev_was_letter = chars[0].is_alphabetic();
    for i in 1..n {
        let c = chars[i];
        if is_terminal(c) {
            if !prev_was_letter {
                return false; // Rule 4
            }
            return i == n - 1;
        }
        if c == ' ' {
            if chars[i - 1] == ' ' {
                return false; // Rule 3: single space
            }
            prev_was_letter = false;
        } else if c.is_lowercase() {
            prev_was_letter = true;
        } else if is_separator(c) {
            prev_was_letter = false;
        } else {
            return false;
        }
    }
    false // no terminal mark
}

fn main() {
    let tests = [
        "The quick brown fox.",
        "Hello world!",
        "lowercase start.",
        "No terminal mark",
        "Two  spaces here.",
    ];
    for t in &tests {
        if is_valid_sentence(t) {
            println!("{}", t);
        }
    }
}
