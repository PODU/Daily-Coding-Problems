// Day 665: URL shortener. Base62-encode an incrementing counter into a 6-char code;
// dedup with url->code map so the same URL maps once. shorten/restore O(1) avg.
use std::collections::HashMap;

const ALPHA: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

struct Shortener {
    code2url: HashMap<String, String>,
    url2code: HashMap<String, String>,
    counter: u64,
}

impl Shortener {
    fn new() -> Self {
        Shortener { code2url: HashMap::new(), url2code: HashMap::new(), counter: 916132831 }
    }
    fn encode(&self, mut n: u64) -> String {
        let mut b = [0u8; 6];
        for i in (0..6).rev() {
            b[i] = ALPHA[(n % 62) as usize];
            n /= 62;
        }
        String::from_utf8(b.to_vec()).unwrap()
    }
    fn shorten(&mut self, url: &str) -> String {
        if let Some(c) = self.url2code.get(url) {
            return c.clone();
        }
        let code = self.encode(self.counter);
        self.counter += 1;
        self.code2url.insert(code.clone(), url.to_string());
        self.url2code.insert(url.to_string(), code.clone());
        code
    }
    fn restore(&self, code: &str) -> Option<&String> {
        self.code2url.get(code)
    }
}

fn main() {
    let mut s = Shortener::new();
    let c = s.shorten("https://example.com/long/path");
    println!("short: {}", c);
    println!("restore: {:?}", s.restore(&c));
    println!("restore(zzzzzz): {:?}", s.restore("zzzzzz")); // None
}
