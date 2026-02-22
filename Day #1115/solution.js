// Day 1115 - Uniform random in [0, n) excluding list l
// Approach: remap |E| excluded slots below m=n-|E| to available high slots,
// then sample once in [0, m). Time: O(|E|) prep, O(1)/sample.

class Sampler {
  constructor(n, l) {
    this.n = n;
    const excluded = new Set(l.filter(x => x >= 0 && x < n));
    this.m = n - excluded.size;
    const available = [];
    for (let v = this.m; v < n; v++) if (!excluded.has(v)) available.push(v);
    this.mapping = new Map();
    let ai = 0;
    for (const e of [...excluded].sort((a, b) => a - b)) {
      if (e < this.m) this.mapping.set(e, available[ai++]);
    }
  }
  sample() {
    const r = Math.floor(Math.random() * this.m);
    return this.mapping.has(r) ? this.mapping.get(r) : r;
  }
}

const n = 10, l = [2, 5, 7];
const s = new Sampler(n, l);
const seen = new Set();
for (let i = 0; i < 2000; i++) seen.add(s.sample());
console.log("n=10, excluded=[2, 5, 7]");
console.log("Sampled valid numbers: [" + [...seen].sort((a, b) => a - b).join(", ") + "]");
