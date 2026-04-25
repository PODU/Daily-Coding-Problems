// Day 1424: URL shortener (shorten -> 6-char code, restore -> original or null).
// Approach: two hash maps + base62 counter; same URL reuses its code. O(1) amortized per op.

const ALPHABET = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

class URLShortener {
  constructor() {
    this.counter = 0;
    this.urlToShort = new Map();
    this.shortToUrl = new Map();
  }
  _encode(n) {
    const s = new Array(6).fill("0");
    for (let i = 5; i >= 0; i--) {
      s[i] = ALPHABET[n % 62];
      n = Math.floor(n / 62);
    }
    return s.join("");
  }
  shorten(url) {
    if (this.urlToShort.has(url)) return this.urlToShort.get(url); // same URL -> same code
    const code = this._encode(this.counter++);
    this.urlToShort.set(url, code);
    this.shortToUrl.set(code, url);
    return code;
  }
  restore(code) {
    return this.shortToUrl.has(code) ? this.shortToUrl.get(code) : null;
  }
}

const s = new URLShortener();
const a = s.shorten("https://example.com/page");
const b = s.shorten("https://example.com/page");
console.log(a);              // 000000
console.log(a === b);        // true
console.log(s.restore(a));   // https://example.com/page
console.log(s.restore("zzzzzz")); // null
