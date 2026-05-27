// URL shortener: base62 6-char codes from a counter, two maps (forward + reverse) for dedupe.
// Time O(1) per shorten/restore, space O(n).
use std::collections::HashMap;

const ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct URLShortener {
    counter: u64,
    code_to_url: HashMap<String, String>,
    url_to_code: HashMap<String, String>,
}

impl URLShortener {
    fn new() -> Self {
        let mut u = URLShortener {
            counter: 0,
            code_to_url: HashMap::new(),
            url_to_code: HashMap::new(),
        };
        u.counter = Self::decode("abcdef");
        u
    }

    fn encode(mut num: u64) -> String {
        let mut b = [b'0'; 6];
        for i in (0..6).rev() {
            b[i] = ALPHABET[(num % 62) as usize];
            num /= 62;
        }
        String::from_utf8(b.to_vec()).unwrap()
    }

    fn decode(s: &str) -> u64 {
        let mut num = 0u64;
        for c in s.bytes() {
            let idx = ALPHABET.iter().position(|&x| x == c).unwrap() as u64;
            num = num * 62 + idx;
        }
        num
    }

    fn shorten(&mut self, url: &str) -> String {
        if let Some(code) = self.url_to_code.get(url) {
            return code.clone();
        }
        let code = Self::encode(self.counter);
        self.counter += 1;
        self.code_to_url.insert(code.clone(), url.to_string());
        self.url_to_code.insert(url.to_string(), code.clone());
        code
    }

    fn restore(&self, code: &str) -> Option<&String> {
        self.code_to_url.get(code)
    }
}

fn main() {
    let mut s = URLShortener::new();
    let code = s.shorten("https://www.example.com/some/long/path");
    println!("{}", code);
    match s.restore(&code) {
        Some(url) => println!("{}", url),
        None => println!("null"),
    }
    match s.restore("XXXXXX") {
        Some(url) => println!("{}", url),
        None => println!("null"),
    }
}
