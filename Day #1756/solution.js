// Day 1756: HitCounter design.
// Store timestamps in a sorted array; total() O(1), range() via binary search O(log n).
// Limited-memory follow-up: bucket hits by coarse time granularity (e.g. per-second
// counts in a map/ring buffer) so memory is O(#buckets) instead of O(#hits).
'use strict';

class HitCounter {
  constructor() {
    this.hits = []; // kept sorted
  }
  record(timestamp) {
    const idx = this._upperBound(timestamp);
    this.hits.splice(idx, 0, timestamp);
  }
  total() {
    return this.hits.length;
  }
  range(lower, upper) {
    return this._upperBound(upper) - this._lowerBound(lower);
  }
  // first index with value > key
  _upperBound(key) {
    let lo = 0, hi = this.hits.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (this.hits[mid] <= key) lo = mid + 1; else hi = mid;
    }
    return lo;
  }
  // first index with value >= key
  _lowerBound(key) {
    let lo = 0, hi = this.hits.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (this.hits[mid] < key) lo = mid + 1; else hi = mid;
    }
    return lo;
  }
}

function main() {
  const hc = new HitCounter();
  for (const t of [1, 2, 2, 5, 7, 9, 10]) hc.record(t);

  console.log('total() =', hc.total());           // 7
  console.log('range(2, 7) =', hc.range(2, 7));    // 4
  console.log('range(0, 10) =', hc.range(0, 10));  // 7
}

main();
