// Rotate array right by k via three reversals: reverse all, reverse first k, reverse rest.
// Time: O(n), Space: O(1).
'use strict';

function reverse(a, i, j) {
    while (i < j) {
        const t = a[i];
        a[i] = a[j];
        a[j] = t;
        i++;
        j--;
    }
}

function rotate(a, k) {
    const n = a.length;
    if (n === 0) return;
    k %= n;
    reverse(a, 0, n - 1);
    reverse(a, 0, k - 1);
    reverse(a, k, n - 1);
}

function main() {
    const a = [1, 2, 3, 4, 5, 6, 7];
    rotate(a, 3);
    console.log(a.join(' '));
}

main();
