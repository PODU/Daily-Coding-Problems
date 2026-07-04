// Soundex phonetic encoding (NARA rules): keep first letter, map rest to digits,
// collapse adjacent same-codes (h/w transparent), drop vowels, pad/truncate to 3 digits.
// Time: O(n) per name, Space: O(n).

fn code(c: char) -> u8 {
    match c.to_ascii_lowercase() {
        'b' | 'f' | 'p' | 'v' => 1,
        'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => 2,
        'd' | 't' => 3,
        'l' => 4,
        'm' | 'n' => 5,
        'r' => 6,
        _ => 0, // vowels a,e,i,o,u,y and h,w
    }
}

fn soundex(name: &str) -> String {
    let s: Vec<char> = name.chars().filter(|c| c.is_alphabetic()).collect();
    if s.is_empty() {
        return String::new();
    }
    let mut res = String::new();
    res.push(s[0].to_ascii_uppercase());
    let mut prev = code(s[0]);
    for &ch in s.iter().skip(1) {
        if res.len() >= 4 {
            break;
        }
        let d = code(ch);
        if d != 0 && d != prev {
            res.push((b'0' + d) as char);
        }
        let lc = ch.to_ascii_lowercase();
        if lc != 'h' && lc != 'w' {
            prev = d;
        }
    }
    while res.len() < 4 {
        res.push('0');
    }
    res.truncate(4);
    res
}

fn main() {
    println!("{}", soundex("Jackson"));
    println!("{}", soundex("Jaxen"));
}
