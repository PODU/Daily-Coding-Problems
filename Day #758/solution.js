// Day 758: Rotate a list left by k in place using the 3-reversal trick.
// No copy; ~n swaps total (each reversal swaps floor(len/2) pairs).
// Time: O(n), Space: O(1).
"use strict";

function reverseRange(a, i, j) {
    let swaps = 0;
    while (i < j) {
        const t = a[i]; a[i] = a[j]; a[j] = t;
        i++; j--; swaps++;
    }
    return swaps;
}

function rotateLeft(a, k) {
    const n = a.length;
    if (n === 0) return 0;
    k %= n;
    let swaps = 0;
    swaps += reverseRange(a, 0, k - 1);
    swaps += reverseRange(a, k, n - 1);
    swaps += reverseRange(a, 0, n - 1);
    return swaps;
}

const a = [1, 2, 3, 4, 5, 6];
const swaps = rotateLeft(a, 2);
console.log("[" + a.join(", ") + "]");  // [3, 4, 5, 6, 1, 2]
console.log("swaps: " + swaps);
