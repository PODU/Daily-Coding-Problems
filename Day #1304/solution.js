// Day 1304: Uniformly sample an integer in [0, n) not in list l.
// Precompute sorted excluded; pick r in [0, n-|excl|) and map to the r-th allowed value.
// Preprocess O(m log m); each draw O(m). Uniform over all allowed values.
class RandExcluder {
  constructor(n, l) {
    this.n = n;
    this.excl = [...new Set(l.filter((x) => x >= 0 && x < n))].sort(
      (a, b) => a - b
    );
  }
  next() {
    const count = this.n - this.excl.length;
    let res = Math.floor(Math.random() * count);
    for (const e of this.excl) {
      if (e <= res) res++;
      else break;
    }
    return res;
  }
}

const r = new RandExcluder(5, [1, 3]);
const seen = new Set();
for (let i = 0; i < 1000; i++) seen.add(r.next());
console.log([...seen].sort((a, b) => a - b)); // [0, 2, 4]
