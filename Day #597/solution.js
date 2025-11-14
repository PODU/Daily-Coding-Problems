// Day 597: Detect a Pythagorean triplet a^2 + b^2 = c^2 in an array.
// Approach: square values, sort, fix c as the largest, two-pointer. Time O(n^2), Space O(n).
'use strict';

function hasPythagoreanTriplet(nums) {
    const sq = nums.map(v => v * v).sort((a, b) => a - b);
    const n = sq.length;
    for (let c = n - 1; c >= 2; c--) {
        let lo = 0, hi = c - 1;
        while (lo < hi) {
            const s = sq[lo] + sq[hi];
            if (s === sq[c]) return true;
            else if (s < sq[c]) lo++;
            else hi--;
        }
    }
    return false;
}

function main() {
    const arr = [3, 1, 4, 6, 5];   // contains 3,4,5
    console.log(hasPythagoreanTriplet(arr));
}

main();
