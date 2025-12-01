// Next permutation of digits: find pivot, swap with next-larger suffix digit,
// reverse suffix. Time: O(d) digits, Space: O(d).
'use strict';

function nextPermutation(num) {
    const d = String(num).split('');
    const n = d.length;
    let i = n - 2;
    while (i >= 0 && d[i] >= d[i + 1]) i--;
    if (i < 0) return num; // already largest permutation
    let j = n - 1;
    while (d[j] <= d[i]) j--;
    [d[i], d[j]] = [d[j], d[i]];
    let l = i + 1, r = n - 1;
    while (l < r) { [d[l], d[r]] = [d[r], d[l]]; l++; r--; }
    return Number(d.join(''));
}

function main() {
    console.log(nextPermutation(48975));
}

main();
