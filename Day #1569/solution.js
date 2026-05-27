// URL shortener: base62 6-char codes from a counter, two maps (forward + reverse) for dedupe.
// Time O(1) per shorten/restore, space O(n).
const ALPHABET = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

class URLShortener {
  constructor() {
    this.counter = this._decode("abcdef");
    this.codeToUrl = new Map();
    this.urlToCode = new Map();
  }
  _encode(num) {
    const s = new Array(6).fill("0");
    for (let i = 5; i >= 0; i--) {
      s[i] = ALPHABET[num % 62];
      num = Math.floor(num / 62);
    }
    return s.join("");
  }
  _decode(s) {
    let num = 0;
    for (const c of s) num = num * 62 + ALPHABET.indexOf(c);
    return num;
  }
  shorten(url) {
    if (this.urlToCode.has(url)) return this.urlToCode.get(url);
    const code = this._encode(this.counter);
    this.counter += 1;
    this.codeToUrl.set(code, url);
    this.urlToCode.set(url, code);
    return code;
  }
  restore(code) {
    return this.codeToUrl.has(code) ? this.codeToUrl.get(code) : null;
  }
}

const s = new URLShortener();
const code = s.shorten("https://www.example.com/some/long/path");
console.log(code);
const r1 = s.restore(code);
console.log(r1 === null ? "null" : r1);
const r2 = s.restore("XXXXXX");
console.log(r2 === null ? "null" : r2);
