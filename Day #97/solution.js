// Day 97: Time-versioned map. Each key keeps times[]/vals[] sorted by time; get
// binary-searches the latest time <= query. set/get O(log n) (insert amortized).
class TimeMap {
  constructor() { this.store = new Map(); } // key -> {times:[], vals:[]}

  set(key, value, time) {
    if (!this.store.has(key)) this.store.set(key, { times: [], vals: [] });
    const { times, vals } = this.store.get(key);
    let lo = 0, hi = times.length;
    while (lo < hi) { const m = (lo + hi) >> 1; times[m] < time ? lo = m + 1 : hi = m; }
    if (lo < times.length && times[lo] === time) vals[lo] = value;
    else { times.splice(lo, 0, time); vals.splice(lo, 0, value); }
  }

  get(key, time) {
    if (!this.store.has(key)) return null;
    const { times, vals } = this.store.get(key);
    let lo = 0, hi = times.length; // first index with time > query
    while (lo < hi) { const m = (lo + hi) >> 1; times[m] <= time ? lo = m + 1 : hi = m; }
    return lo > 0 ? vals[lo - 1] : null;
  }
}

// The README's three blocks are independent scenarios (fresh maps).
const a = new TimeMap();
a.set(1, 1, 0); a.set(1, 2, 2);
console.log(a.get(1, 1));   // 1
console.log(a.get(1, 3));   // 2

const b = new TimeMap();
b.set(1, 1, 5);
console.log(b.get(1, 0));   // null
console.log(b.get(1, 10));  // 1

const c = new TimeMap();
c.set(1, 1, 0); c.set(1, 2, 0);
console.log(c.get(1, 0));   // 2
