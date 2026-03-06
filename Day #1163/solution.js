// Greedy: sort intervals by end ascending; keep if start >= last kept end; count removals. O(n log n) time, O(1) extra space.
'use strict';

function eraseOverlapIntervals(intervals) {
    intervals.sort((a, b) => a[1] - b[1]);
    let removals = 0;
    let lastEnd = -Infinity;
    for (const [start, end] of intervals) {
        if (start >= lastEnd) {
            lastEnd = end;
        } else {
            removals++;
        }
    }
    return removals;
}

const intervals = [[7, 9], [2, 4], [5, 8]];
console.log(eraseOverlapIntervals(intervals));
