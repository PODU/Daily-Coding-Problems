// Day 607: Min total movement to seat M people contiguously in a row.
// Approach: target = median of (pos[i]-i); cost = sum |(pos[i]-i) - median|. Time O(n), Space O(M).
'use strict';

function minCost(seats) {
    const b = [];
    let idx = 0;
    for (let i = 0; i < seats.length; i++)
        if (seats[i] === 1) b.push(i - idx++);
    if (b.length === 0) return 0;
    b.sort((a, c) => a - c);
    const med = b[b.length >> 1];
    return b.reduce((acc, v) => acc + Math.abs(v - med), 0);
}

function main() {
    const seats = [0, 1, 1, 0, 1, 0, 0, 0, 1];
    console.log(minCost(seats)); // 5
}

main();
