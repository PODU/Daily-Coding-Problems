// Day 132: HitCounter (record, total, range).
// Keep timestamps sorted; range via binary search. total O(1), range O(log n).
// Limited-memory follow-up: bucket counts by coarse time granularity instead of per-hit.
function lowerBound(a, x) {
  let lo = 0, hi = a.length;
  while (lo < hi) {
    const m = (lo + hi) >> 1;
    if (a[m] < x) lo = m + 1;
    else hi = m;
  }
  return lo;
}
function upperBound(a, x) {
  let lo = 0, hi = a.length;
  while (lo < hi) {
    const m = (lo + hi) >> 1;
    if (a[m] <= x) lo = m + 1;
    else hi = m;
  }
  return lo;
}

class HitCounter {
  constructor() {
    this.ts = [];
  }
  record(t) {
    this.ts.splice(upperBound(this.ts, t), 0, t);
  }
  total() {
    return this.ts.length;
  }
  range(lo, hi) {
    return upperBound(this.ts, hi) - lowerBound(this.ts, lo);
  }
}

const hc = new HitCounter();
for (const t of [1, 1, 2, 3, 5, 8, 8, 10]) hc.record(t);
console.log("total = " + hc.total());          // 8
console.log("range(2, 8) = " + hc.range(2, 8)); // 5
