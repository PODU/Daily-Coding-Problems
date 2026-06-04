// Skyline via sweep-line + multiset of active heights. Emit point when current max changes.
// Time: O(n log n) with a balanced structure; here O(n^2) worst via sorted array kept simple.
// Uses a count map + tracked max for clarity; still O(n log n) on the sort of events.
'use strict';

function getSkyline(buildings) {
    const events = []; // [x, signed height] start = -h, end = +h
    for (const [left, right, h] of buildings) {
        events.push([left, -h]);
        events.push([right, h]);
    }
    events.sort((a, b) => a[0] - b[0] || a[1] - b[1]);

    const counts = new Map();
    counts.set(0, 1);
    const curMax = () => {
        let m = 0;
        for (const k of counts.keys()) if (k > m) m = k;
        return m;
    };

    let prev = 0;
    const res = [];
    for (const [x, h] of events) {
        if (h < 0) {
            const k = -h;
            counts.set(k, (counts.get(k) || 0) + 1);
        } else {
            const c = counts.get(h);
            if (c === 1) counts.delete(h); else counts.set(h, c - 1);
        }
        const cur = curMax();
        if (cur !== prev) {
            res.push([x, cur]);
            prev = cur;
        }
    }
    return res;
}

function main() {
    const buildings = [[0, 15, 3], [4, 11, 5], [19, 23, 4]];
    const res = getSkyline(buildings);
    console.log("[" + res.map(([x, h]) => `(${x}, ${h})`).join(", ") + "]");
}

main();
