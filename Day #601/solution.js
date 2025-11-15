// Day 601: Reconstruct a permutation of [0..N] matching +/- relations between neighbors.
// Approach: greedy low/high pointers (DI-match). Time O(n), Space O(n). Any consistent array is valid.
'use strict';

function reconstruct(signs) {
    // signs[0] is null; signs[k] is '+' if a[k] > a[k-1], else '-'.
    const n = signs.length;       // numbers 0..n-1
    let low = 0, high = n - 1;
    const res = [];
    for (let k = 1; k < n; k++) {
        if (signs[k] === '+') res.push(low++);
        else res.push(high--);
    }
    res.push(low);
    return res;
}

function main() {
    const signs = [null, '+', '+', '-', '+'];
    console.log('[' + reconstruct(signs).join(', ') + ']');
}

main();
