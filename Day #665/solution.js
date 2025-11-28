// Day 665: URL shortener. Base62-encode an incrementing counter into a 6-char code;
// dedup with url->code map so the same URL maps once. shorten/restore O(1) avg.
const A = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

class Shortener {
  constructor() { this.code2url = new Map(); this.url2code = new Map(); this.counter = 916132831; }
  encode(n) {
    let s = "";
    for (let i = 0; i < 6; i++) { s = A[n % 62] + s; n = Math.floor(n / 62); }
    return s;
  }
  shorten(url) {
    if (this.url2code.has(url)) return this.url2code.get(url);
    const code = this.encode(this.counter++);
    this.code2url.set(code, url); this.url2code.set(url, code);
    return code;
  }
  restore(code) { return this.code2url.has(code) ? this.code2url.get(code) : null; }
}

const s = new Shortener();
const c = s.shorten("https://example.com/long/path");
console.log("short:", c);
console.log("restore:", s.restore(c));
console.log("restore(zzzzzz):", s.restore("zzzzzz")); // null
