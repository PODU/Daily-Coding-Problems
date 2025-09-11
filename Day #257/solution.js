// Day 257: Smallest window that must be sorted to make the whole array sorted.
// Left->right track max to find right bound; right->left track min to find left bound.
// Time: O(n), Space: O(1).
"use strict";

function sortWindow(a) {
    const n = a.length;
    let begin = -1, end = -1;
    let mx = -Infinity;
    for (let i = 0; i < n; i++) {
        if (a[i] < mx) end = i;
        else mx = a[i];
    }
    let mn = Infinity;
    for (let i = n - 1; i >= 0; i--) {
        if (a[i] > mn) begin = i;
        else mn = a[i];
    }
    return [begin, end];
}

function main() {
    const a = [3, 7, 5, 6, 9];
    const [b, e] = sortWindow(a);
    console.log(`(${b}, ${e})`); // (1, 3)
}

main();
