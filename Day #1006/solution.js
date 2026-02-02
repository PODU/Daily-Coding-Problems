// Count intersecting segment pairs: sort by p, count inversions in q via merge sort.
// Time O(n log n), Space O(n).
'use strict';

function mergeCount(a) {
    if (a.length <= 1) return [a, 0];
    const m = a.length >> 1;
    const [left, cl] = mergeCount(a.slice(0, m));
    const [right, cr] = mergeCount(a.slice(m));
    const merged = [];
    let i = 0, j = 0, cnt = cl + cr;
    while (i < left.length && j < right.length) {
        if (left[i] <= right[j]) merged.push(left[i++]);
        else { merged.push(right[j++]); cnt += left.length - i; }
    }
    while (i < left.length) merged.push(left[i++]);
    while (j < right.length) merged.push(right[j++]);
    return [merged, cnt];
}

function countIntersections(p, q) {
    const order = [...p.keys()].sort((x, y) => p[x] - p[y]);
    const qs = order.map(i => q[i]);
    return mergeCount(qs)[1];
}

const p = [1, 2, 3, 4];
const q = [4, 3, 2, 1];
console.log(countIntersections(p, q)); // 6
