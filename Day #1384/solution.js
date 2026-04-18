// HitCounter: keep timestamps in a sorted array (binary-insert); record O(n), total O(1), range via binary search (upper-lower).
class HitCounter {
  constructor() {
    this.hits = [];
    this.cnt = 0;
  }

  _lowerBound(key) {
    let lo = 0, hi = this.hits.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (this.hits[mid] < key) lo = mid + 1;
      else hi = mid;
    }
    return lo;
  }

  _upperBound(key) {
    let lo = 0, hi = this.hits.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (this.hits[mid] <= key) lo = mid + 1;
      else hi = mid;
    }
    return lo;
  }

  record(t) {
    const i = this._lowerBound(t);
    this.hits.splice(i, 0, t);
    this.cnt++;
  }

  total() { return this.cnt; }

  range(lo, hi) { return this._upperBound(hi) - this._lowerBound(lo); }
}

const hc = new HitCounter();
for (const t of [1, 1, 2, 3, 5, 8]) hc.record(t);
console.log(`total: ${hc.total()}`);
console.log(`range(2,5): ${hc.range(2, 5)}`);
