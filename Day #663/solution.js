// Day 663: HitCounter. Keep timestamps sorted; total = O(1); range = binary search.
// record O(n) sorted insert, total O(1), range O(log n).
// Limited-memory follow-up: bucket hits into fixed time windows storing (window, count).
class HitCounter {
  constructor() { this.ts = []; }
  _lb(x) { let lo = 0, hi = this.ts.length; while (lo < hi) { const m = (lo + hi) >> 1; if (this.ts[m] < x) lo = m + 1; else hi = m; } return lo; }
  _ub(x) { let lo = 0, hi = this.ts.length; while (lo < hi) { const m = (lo + hi) >> 1; if (this.ts[m] <= x) lo = m + 1; else hi = m; } return lo; }
  record(t) { this.ts.splice(this._ub(t), 0, t); }
  total() { return this.ts.length; }
  range(lo, hi) { return this._ub(hi) - this._lb(lo); }
}

const h = new HitCounter();
for (const t of [1, 2, 2, 5, 9, 10]) h.record(t);
console.log("total:", h.total());          // 6
console.log("range(2,9):", h.range(2, 9));  // 4
