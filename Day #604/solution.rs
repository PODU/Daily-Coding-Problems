// Day 604: Soundex phonetic encoding (letter + 3 digits).
// Approach: keep first letter, code consonants, drop repeats/vowels, pad. Time O(L), Space O(L).

fn code(c: char) -> char {
    match c {
        'B' | 'F' | 'P' | 'V' => '1',
        'C' | 'G' | 'J' | 'K' | 'Q' | 'S' | 'X' | 'Z' => '2',
        'D' | 'T' => '3',
        'L' => '4',
        'M' | 'N' => '5',
        'R' => '6',
        'H' | 'W' => 'S', // transparent
        _ => '0',         // vowels
    }
}

fn soundex(name: &str) -> String {
    let up: Vec<char> = name
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();
    if up.is_empty() {
        return "0000".to_string();
    }
    let mut res = String::new();
    res.push(up[0]);
    let mut prev = code(up[0]);
    for &ch in up.iter().skip(1) {
        if res.len() >= 4 {
            break;
        }
        let c = code(ch);
        if ('1'..='6').contains(&c) {
            if c != prev {
                res.push(c);
            }
            prev = c;
        } else if c == '0' {
            prev = '0';
        }
    }
    while res.len() < 4 {
        res.push('0');
    }
    res.chars().take(4).collect()
}

fn main() {
    println!("{}", soundex("Jackson")); // J250
    println!("{}", soundex("Jaxen"));   // J250
}
