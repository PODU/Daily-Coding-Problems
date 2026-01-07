// Soundex phonetic encoding: keep first letter, code rest, dedupe, pad to 3 digits.
// Time O(n), Space O(1) extra per name.

fn code(c: char) -> i32 {
    match c.to_ascii_lowercase() {
        'b' | 'f' | 'p' | 'v' => 1,
        'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => 2,
        'd' | 't' => 3,
        'l' => 4,
        'm' | 'n' => 5,
        'r' => 6,
        _ => 0,
    }
}

fn soundex(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut i = 0;
    while i < chars.len() && !chars[i].is_alphabetic() {
        i += 1;
    }
    if i >= chars.len() {
        return String::new();
    }
    let mut res: Vec<char> = vec![chars[i].to_ascii_uppercase()];
    let mut prev = code(chars[i]);
    let mut j = i + 1;
    while j < chars.len() && res.len() < 4 {
        let c = chars[j].to_ascii_lowercase();
        j += 1;
        if !c.is_alphabetic() {
            continue;
        }
        if c == 'h' || c == 'w' {
            continue;
        }
        let d = code(c);
        if d == 0 {
            prev = 0;
            continue;
        }
        if d != prev {
            res.push((b'0' + d as u8) as char);
        }
        prev = d;
    }
    while res.len() < 4 {
        res.push('0');
    }
    res.into_iter().take(4).collect()
}

fn main() {
    println!("{}", soundex("Jackson"));
    println!("{}", soundex("Jaxen"));
}
