// URL shortener: base62-encode an incrementing counter (zero-padded to 6 chars).
// Dedup via url->code map so identical URLs map to the same code.
// shorten/restore: O(L) per call (L = code length); Space: O(N) for N stored URLs.
"use strict";

const ALPHA = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

class URLShortener {
  constructor() {
    this.counter = 0;
    this.urlToCode = new Map();
    this.codeToUrl = new Map();
  }

  _encode(n) {
    const s = new Array(6).fill(ALPHA[0]);
    let i = 5;
    while (n > 0 && i >= 0) {
      s[i--] = ALPHA[n % 62];
      n = Math.floor(n / 62);
    }
    return s.join("");
  }

  shorten(url) {
    if (this.urlToCode.has(url)) return this.urlToCode.get(url); // same URL -> same code
    const code = this._encode(this.counter++);
    this.urlToCode.set(url, code);
    this.codeToUrl.set(code, url);
    return code;
  }

  restore(short) {
    return this.codeToUrl.has(short) ? this.codeToUrl.get(short) : null;
  }
}

const s = new URLShortener();
const url = "https://www.example.com/some/long/path";
const code = s.shorten(url);
console.log(`shorten(${url}) = ${code}`);
console.log(`restore(${code}) = ${s.restore(code)}`);
console.log(`restore(zzzzzz) = ${s.restore("zzzzzz")}`);
const code2 = s.shorten(url);
console.log(`shorten same url again = ${code2} (same as before: ${code === code2})`);
