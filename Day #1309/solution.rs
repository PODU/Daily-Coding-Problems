// Soundex phonetic encoding: keep first letter, map consonants to digits,
// collapse same-code runs, drop vowels, pad/truncate to 3 digits. O(n) time.

fn code(c: char) -> u8 {
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
    if chars.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    out.push(chars[0].to_ascii_uppercase());
    let mut prev = code(chars[0]);
    for &ch in chars.iter().skip(1) {
        if out.len() >= 4 {
            break;
        }
        let lc = ch.to_ascii_lowercase();
        let c = code(lc);
        if c != 0 && c != prev {
            out.push((b'0' + c) as char);
        }
        if lc == 'h' || lc == 'w' {
            continue;
        }
        prev = c;
    }
    while out.len() < 4 {
        out.push('0');
    }
    out.truncate(4);
    out
}

fn main() {
    println!("{}", soundex("Jackson")); // J250
    println!("{}", soundex("Jaxen"));   // J250
}
