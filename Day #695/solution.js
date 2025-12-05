// Day 695: Uniform random in [0, n-1] excluding values in list l.
// Approach: precompute the allowed values once, then pick a uniform index.
// Preprocess O(n), each draw O(1).
class RandExcluder {
  constructor(n, l) {
    const bad = new Set(l);
    this.allowed = [];
    for (let x = 0; x < n; x++) if (!bad.has(x)) this.allowed.push(x);
  }
  next() {
    return this.allowed[Math.floor(Math.random() * this.allowed.length)];
  }
}

const r = new RandExcluder(10, [2, 4, 6, 8]);
const sample = Array.from({ length: 8 }, () => r.next());
console.log("sample:", sample);
console.log("(all values are in [0,9] and never 2,4,6,8)");
