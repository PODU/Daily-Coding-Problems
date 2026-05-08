// Day 1489: Time-indexed map. Per key, keep entries sorted by time; get does
// binary search for the most recent time <= query. set O(log n), get O(log n).

class TimeMap {
  constructor() {
    this.store = new Map(); // key -> sorted array of [time, value]
  }

  set(key, value, time) {
    if (!this.store.has(key)) this.store.set(key, []);
    const v = this.store.get(key);
    // first index with time >= target
    let lo = 0, hi = v.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (v[mid][0] < time) lo = mid + 1; else hi = mid;
    }
    if (lo < v.length && v[lo][0] === time) v[lo][1] = value; // overwrite same time
    else v.splice(lo, 0, [time, value]);
  }

  get(key, time) {
    const v = this.store.get(key);
    if (!v) return null;
    let lo = 0, hi = v.length - 1, idx = -1;
    while (lo <= hi) {
      const mid = (lo + hi) >> 1;
      if (v[mid][0] <= time) { idx = mid; lo = mid + 1; }
      else hi = mid - 1;
    }
    return idx < 0 ? null : v[idx][1];
  }
}

function show(d, key, time) {
  const val = d.get(key, time);
  console.log(`get(${key},${time}) = ${val === null ? "null" : val}`);
}

const d1 = new TimeMap();
d1.set(1, 1, 0); d1.set(1, 2, 2);
show(d1, 1, 1);   // 1
show(d1, 1, 3);   // 2

const d2 = new TimeMap();
d2.set(1, 1, 5);
show(d2, 1, 0);   // null
show(d2, 1, 10);  // 1

const d3 = new TimeMap();
d3.set(1, 1, 0); d3.set(1, 2, 0);
show(d3, 1, 0);   // 2
