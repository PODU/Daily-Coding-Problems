// Day 1718: Count intersecting segment pairs (p_i on y=0 -> q_i on y=1).
// Two segments cross iff their (p, q) ordering is inverted: sort by p,
// count inversions in q via merge sort. Time: O(n log n), Space: O(n).

function mergeCount(a, l, r) {
    if (r - l <= 1) return 0;
    const mid = (l + r) >> 1;
    let inv = mergeCount(a, l, mid) + mergeCount(a, mid, r);
    const tmp = [];
    let i = l, j = mid;
    while (i < mid && j < r) {
        if (a[i] <= a[j]) tmp.push(a[i++]);
        else { inv += mid - i; tmp.push(a[j++]); }
    }
    while (i < mid) tmp.push(a[i++]);
    while (j < r) tmp.push(a[j++]);
    for (let t = 0; t < tmp.length; t++) a[l + t] = tmp[t];
    return inv;
}

function countIntersections(p, q) {
    const n = p.length;
    const idx = [...Array(n).keys()].sort((x, y) => p[x] - p[y]);
    const qs = idx.map((i) => q[i]);
    return mergeCount(qs, 0, n);
}

const p = [1, 2, 3, 4];
const q = [2, 1, 4, 3];
console.log("Intersecting pairs: " + countIntersections(p, q));
