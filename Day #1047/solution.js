// Time-versioned map: per key a sorted array of [time,value]; get binary-searches for floor.
// set/get O(log n) lookup. Setting same key+time overwrites.
'use strict';

class TimeMap {
    constructor() {
        this.store = new Map(); // key -> array of [time, value] sorted by time
    }
    set(key, value, time) {
        if (!this.store.has(key)) this.store.set(key, []);
        const arr = this.store.get(key);
        // binary search for insert position
        let lo = 0, hi = arr.length;
        while (lo < hi) {
            const mid = (lo + hi) >> 1;
            if (arr[mid][0] < time) lo = mid + 1;
            else hi = mid;
        }
        if (lo < arr.length && arr[lo][0] === time) arr[lo][1] = value; // overwrite
        else arr.splice(lo, 0, [time, value]);
    }
    get(key, time) {
        if (!this.store.has(key)) return null;
        const arr = this.store.get(key);
        let lo = 0, hi = arr.length; // upper_bound
        while (lo < hi) {
            const mid = (lo + hi) >> 1;
            if (arr[mid][0] <= time) lo = mid + 1;
            else hi = mid;
        }
        if (lo === 0) return null;
        return arr[lo - 1][1];
    }
}

const show = (v) => (v === null ? "null" : String(v));

// README presents three logical blocks; each starts from its own map state.
let d = new TimeMap();
d.set(1, 1, 0);
d.set(1, 2, 2);
console.log("d.get(1, 1) ->", show(d.get(1, 1)));
console.log("d.get(1, 3) ->", show(d.get(1, 3)));

d = new TimeMap();
d.set(1, 1, 5);
console.log("d.get(1, 0) ->", show(d.get(1, 0)));
console.log("d.get(1, 10) ->", show(d.get(1, 10)));

d = new TimeMap();
d.set(1, 1, 0);
d.set(1, 2, 0); // overwrite same key+time -> value 2
console.log("d.get(1, 0) ->", show(d.get(1, 0)));
