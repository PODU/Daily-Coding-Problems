// Bloom filter: fixed 1000-bit array, 3 hashes via double hashing. add/check.
// add O(k), check O(k); Space O(m bits). check may false-positive, never false-negative.
class Bloom {
  constructor() { this.M = 1000; this.K = 3; this.bits = new Uint8Array(this.M); }
  _h1(s) { let h = 5381n; for (const c of s) h = (h * 33n + BigInt(c.charCodeAt(0))) & 0xFFFFFFFFFFFFFFFFn; return h; }
  _h2(s) { let h = 0n; for (const c of s) h = (h * 131n + BigInt(c.charCodeAt(0))) & 0xFFFFFFFFFFFFFFFFn; return h; }
  add(s) { const a = this._h1(s), b = this._h2(s); for (let i = 0n; i < BigInt(this.K); i++) this.bits[Number((a + i * b) % BigInt(this.M))] = 1; }
  check(s) {
    const a = this._h1(s), b = this._h2(s);
    for (let i = 0n; i < BigInt(this.K); i++) if (!this.bits[Number((a + i * b) % BigInt(this.M))]) return false;
    return true;
  }
}
function main() {
  const bf = new Bloom();
  for (const w of ["apple", "banana", "cat"]) bf.add(w);
  for (const w of ["apple", "banana", "cat", "dog"]) console.log(`check ${w}: ${bf.check(w)}`);
}
main();
