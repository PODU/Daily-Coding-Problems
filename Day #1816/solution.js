// Time-versioned map: per key keep times sorted; get returns value at most-recent time <= t.
// set amortized O(log m), get O(log m) via binary search. Space: O(total entries).
class TimeMap {
  constructor() {
    this.data = new Map(); // key -> array of [time, value] sorted by time
  }
  set(key, value, time) {
    if (!this.data.has(key)) this.data.set(key, []);
    const arr = this.data.get(key);
    // binary search for insertion / overwrite position
    let lo = 0, hi = arr.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (arr[mid][0] < time) lo = mid + 1; else hi = mid;
    }
    if (lo < arr.length && arr[lo][0] === time) arr[lo][1] = value;
    else arr.splice(lo, 0, [time, value]);
  }
  get(key, time) {
    const arr = this.data.get(key);
    if (!arr) return null;
    let lo = 0, hi = arr.length; // first index with time > query
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (arr[mid][0] <= time) lo = mid + 1; else hi = mid;
    }
    return lo === 0 ? null : arr[lo - 1][1];
  }
}

function show(d, key, time) {
  const r = d.get(key, time);
  console.log(`get(${key}, ${time}) = ${r === null ? "null" : r}`);
}

let d = new TimeMap(); d.set(1,1,0); d.set(1,2,2); show(d,1,1); show(d,1,3);  // 1, 2
d = new TimeMap(); d.set(1,1,5); show(d,1,0); show(d,1,10);                    // null, 1
d = new TimeMap(); d.set(1,1,0); d.set(1,2,0); show(d,1,0);                    // 2
