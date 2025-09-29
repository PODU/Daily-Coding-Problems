// Soundex: keep first letter, map consonants to digits, collapse same adjacent codes
// (h,w transparent; vowels reset), pad/truncate to 3 digits. Time O(L), Space O(1).
fn code(c: char) -> u8 {
    match c.to_ascii_lowercase() {
        'b' | 'f' | 'p' | 'v' => 1,
        'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => 2,
        'd' | 't' => 3,
        'l' => 4,
        'm' | 'n' => 5,
        'r' => 6,
        _ => 0, // vowels, y, w, h
    }
}

fn soundex(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut res = String::new();
    res.push(chars[0].to_ascii_uppercase());
    let mut prev = code(chars[0]);
    for &ch in &chars[1..] {
        if res.len() >= 4 {
            break;
        }
        let c = ch.to_ascii_lowercase();
        let d = code(c);
        if d != 0 && d != prev {
            res.push((b'0' + d) as char);
        }
        if c == 'h' || c == 'w' {
            continue; // transparent: keep prev
        }
        prev = d; // vowels reset prev to 0
    }
    while res.len() < 4 {
        res.push('0');
    }
    res.truncate(4);
    res
}

fn main() {
    println!("{}", soundex("Jackson"));
}
