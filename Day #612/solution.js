// Day 612: Min intervals to remove so the rest are non-overlapping (touching allowed).
// Approach: sort by end, greedily keep max non-overlapping; answer = total - kept. Time O(n log n).
'use strict';

function minRemovals(intervals) {
    intervals = intervals.slice().sort((a, b) => a[1] - b[1]);
    let kept = 0, end = -Infinity;
    for (const [s, e] of intervals)
        if (s >= end) { kept++; end = e; }
    return intervals.length - kept;
}

function main() {
    const intervals = [[7, 9], [2, 4], [5, 8]];
    console.log(minRemovals(intervals)); // 1
}

main();
