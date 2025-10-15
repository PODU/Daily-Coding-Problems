// Day 433: Next larger integer with the same number of set bits (Gosper's hack).
// c = n & -n; r = n + c; next = (((r ^ n) >> 2) / c) | r. O(1) time, O(1) space.
function nextSameBits(n) {
    if (n <= 0) return n;
    const c = n & (-n);
    const r = n + c;
    return (((r ^ n) >> 2) / c) | r;
}

const n = 6, m = nextSameBits(n);
console.log(`Input: ${n} (${n.toString(2)} in binary)`);
console.log(`Next: ${m} (${m.toString(2)} in binary)`);
