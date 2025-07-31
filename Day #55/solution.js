// Day 55: URL shortener. 6-char base62 code; same URL maps to same code.
// Time: O(1) amortized per op, Space: O(n).
const ALPHA = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

class URLShortener {
  constructor() {
    this.toLong = new Map();
    this.toShort = new Map();
  }

  randCode() {
    let s = "";
    for (let i = 0; i < 6; i++) s += ALPHA[Math.floor(Math.random() * ALPHA.length)];
    return s;
  }

  shorten(url) {
    if (this.toShort.has(url)) return this.toShort.get(url); // same URL -> same code
    let code;
    do { code = this.randCode(); } while (this.toLong.has(code));
    this.toLong.set(code, url);
    this.toShort.set(url, code);
    return code;
  }

  restore(code) {
    return this.toLong.has(code) ? this.toLong.get(code) : null;
  }
}

const s = new URLShortener();
const a = s.shorten("https://example.com/foo");
const b = s.shorten("https://example.com/foo"); // same URL twice
console.log("same code reused:", a === b);
console.log("restore:", s.restore(a));
console.log("restore unknown:", s.restore("zzzzzz"));
