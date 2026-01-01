// Day 837: Sentence checker over a character stream.
// Accumulate chars until a terminal mark, then validate the buffered sentence by hand-rolled FSM; print if valid.
// O(N) over the stream; O(L) per sentence buffer.

fn is_lower(c: char) -> bool {
    c.is_ascii_lowercase()
}
fn is_sep(c: char) -> bool {
    c == ',' || c == ';' || c == ':'
}

// Validate body (terminal already stripped): ^[A-Z][a-z]*[,;:]?( [a-z]+[,;:]?)*$
fn valid_body(body: &str) -> bool {
    let cs: Vec<char> = body.chars().collect();
    let n = cs.len();
    if n == 0 || !cs[0].is_ascii_uppercase() {
        return false;
    }
    let mut i = 1;
    while i < n && is_lower(cs[i]) {
        i += 1;
    }
    if i < n && is_sep(cs[i]) {
        i += 1;
    }
    while i < n {
        if cs[i] != ' ' {
            return false;
        }
        i += 1;
        let start = i;
        while i < n && is_lower(cs[i]) {
            i += 1;
        }
        if i == start {
            return false;
        }
        if i < n && is_sep(cs[i]) {
            i += 1;
        }
    }
    true
}

fn check_stream(stream: &str) -> Vec<String> {
    let terminals = ['.', '?', '!', '‽'];
    let mut results = Vec::new();
    let mut buf = String::new();
    for ch in stream.chars() {
        if terminals.contains(&ch) {
            let sentence: String = buf.trim_start_matches(' ').to_string();
            if valid_body(&sentence) {
                results.push(format!("{}{}", sentence, ch));
            }
            buf.clear();
        } else {
            buf.push(ch);
        }
    }
    results
}

fn main() {
    let stream = "Hello, world. this is wrong. The cat sat.";
    for s in check_stream(stream) {
        println!("{}", s);
    }
}
