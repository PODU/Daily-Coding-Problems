// Day 1424: URL shortener (shorten -> 6-char code, restore -> original or None).
// Approach: two hash maps + base62 counter; same URL reuses its code. O(1) amortized per op.
use std::collections::HashMap;

const ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct URLShortener {
    counter: u64,
    url_to_short: HashMap<String, String>,
    short_to_url: HashMap<String, String>,
}

impl URLShortener {
    fn new() -> Self {
        URLShortener {
            counter: 0,
            url_to_short: HashMap::new(),
            short_to_url: HashMap::new(),
        }
    }
    fn encode(&self, mut n: u64) -> String {
        let mut s = [b'0'; 6];
        for i in (0..6).rev() {
            s[i] = ALPHABET[(n % 62) as usize];
            n /= 62;
        }
        String::from_utf8(s.to_vec()).unwrap()
    }
    fn shorten(&mut self, url: &str) -> String {
        if let Some(code) = self.url_to_short.get(url) {
            return code.clone(); // same URL -> same code
        }
        let code = self.encode(self.counter);
        self.counter += 1;
        self.url_to_short.insert(url.to_string(), code.clone());
        self.short_to_url.insert(code.clone(), url.to_string());
        code
    }
    fn restore(&self, code: &str) -> Option<&String> {
        self.short_to_url.get(code)
    }
}

fn main() {
    let mut s = URLShortener::new();
    let a = s.shorten("https://example.com/page");
    let b = s.shorten("https://example.com/page");
    println!("{}", a);        // 000000
    println!("{}", a == b);   // true
    match s.restore(&a) {
        Some(u) => println!("{}", u), // https://example.com/page
        None => println!("null"),
    }
    match s.restore("zzzzzz") {
        Some(u) => println!("{}", u),
        None => println!("null"), // null
    }
}
