// Day 734: Time-travel map; get(key,t) returns value set at the most recent time <= t.
// Approach: per key a sorted array of [time, value]; get uses binary search.
// Time: set O(n) insert, get O(log n). Space: O(n).

class TimeMap {
  constructor() { this.store = new Map(); }

  set(key, value, time) {
    if (!this.store.has(key)) this.store.set(key, []);
    const arr = this.store.get(key);
    // binary search for time, overwrite if present, else insert keeping sorted
    let lo = 0, hi = arr.length;
    while (lo < hi) { const mid = (lo + hi) >> 1; if (arr[mid][0] < time) lo = mid + 1; else hi = mid; }
    if (lo < arr.length && arr[lo][0] === time) arr[lo][1] = value;
    else arr.splice(lo, 0, [time, value]);
  }

  get(key, time) {
    if (!this.store.has(key)) return null;
    const arr = this.store.get(key);
    let lo = 0, hi = arr.length; // upper bound
    while (lo < hi) { const mid = (lo + hi) >> 1; if (arr[mid][0] <= time) lo = mid + 1; else hi = mid; }
    return lo === 0 ? null : arr[lo - 1][1];
  }
}

const show = (v) => console.log(v === null ? "null" : v);

const d1 = new TimeMap(); d1.set(1, 1, 0); d1.set(1, 2, 2);
show(d1.get(1, 1));  // 1
show(d1.get(1, 3));  // 2
const d2 = new TimeMap(); d2.set(1, 1, 5);
show(d2.get(1, 0));  // null
show(d2.get(1, 10)); // 1
const d3 = new TimeMap(); d3.set(1, 1, 0); d3.set(1, 2, 0);
show(d3.get(1, 0));  // 2
