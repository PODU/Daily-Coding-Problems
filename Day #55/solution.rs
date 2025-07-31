// Day 55: URL shortener. 6-char base62 code; same URL maps to same code.
// Time: O(1) amortized per op, Space: O(n).
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const ALPHA: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

struct Rng(u64);
impl Rng {
    fn new() -> Self {
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        Rng(seed | 1)
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

struct URLShortener {
    to_long: HashMap<String, String>,
    to_short: HashMap<String, String>,
    rng: Rng,
}

impl URLShortener {
    fn new() -> Self {
        URLShortener { to_long: HashMap::new(), to_short: HashMap::new(), rng: Rng::new() }
    }

    fn rand_code(&mut self) -> String {
        (0..6)
            .map(|_| ALPHA[(self.rng.next() % ALPHA.len() as u64) as usize] as char)
            .collect()
    }

    fn shorten(&mut self, url: &str) -> String {
        if let Some(code) = self.to_short.get(url) {
            return code.clone(); // same URL -> same code
        }
        let mut code = self.rand_code();
        while self.to_long.contains_key(&code) {
            code = self.rand_code();
        }
        self.to_long.insert(code.clone(), url.to_string());
        self.to_short.insert(url.to_string(), code.clone());
        code
    }

    fn restore(&self, code: &str) -> Option<&String> {
        self.to_long.get(code) // None if unknown
    }
}

fn main() {
    let mut s = URLShortener::new();
    let a = s.shorten("https://example.com/foo");
    let b = s.shorten("https://example.com/foo"); // same URL twice
    println!("same code reused: {}", a == b);
    println!("restore: {}", s.restore(&a).unwrap());
    match s.restore("zzzzzz") {
        Some(u) => println!("restore unknown: {}", u),
        None => println!("restore unknown: null"),
    }
}
