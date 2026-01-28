// URL shortener: base62-encode an incrementing counter (zero-padded to 6 chars).
// Dedup via url->code map so identical URLs map to the same code.
// shorten/restore: O(L) per call (L = code length); Space: O(N) for N stored URLs.
use std::collections::HashMap;

const ALPHA: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct UrlShortener {
    counter: u64,
    url_to_code: HashMap<String, String>,
    code_to_url: HashMap<String, String>,
}

impl UrlShortener {
    fn new() -> Self {
        UrlShortener {
            counter: 0,
            url_to_code: HashMap::new(),
            code_to_url: HashMap::new(),
        }
    }

    fn encode(mut n: u64) -> String {
        let mut s = [ALPHA[0]; 6];
        let mut i: i32 = 5;
        while n > 0 && i >= 0 {
            s[i as usize] = ALPHA[(n % 62) as usize];
            n /= 62;
            i -= 1;
        }
        String::from_utf8(s.to_vec()).unwrap()
    }

    fn shorten(&mut self, url: &str) -> String {
        if let Some(code) = self.url_to_code.get(url) {
            return code.clone(); // same URL -> same code
        }
        let code = Self::encode(self.counter);
        self.counter += 1;
        self.url_to_code.insert(url.to_string(), code.clone());
        self.code_to_url.insert(code.clone(), url.to_string());
        code
    }

    fn restore(&self, short: &str) -> Option<&String> {
        self.code_to_url.get(short) // None if unknown
    }
}

fn main() {
    let mut s = UrlShortener::new();
    let url = "https://www.example.com/some/long/path";
    let code = s.shorten(url);
    println!("shorten({}) = {}", url, code);

    match s.restore(&code) {
        Some(v) => println!("restore({}) = {}", code, v),
        None => println!("restore({}) = null", code),
    }
    match s.restore("zzzzzz") {
        Some(v) => println!("restore(zzzzzz) = {}", v),
        None => println!("restore(zzzzzz) = null"),
    }
    let code2 = s.shorten(url);
    println!("shorten same url again = {} (same as before: {})", code2, code == code2);
}
