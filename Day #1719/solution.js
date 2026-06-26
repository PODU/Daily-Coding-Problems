// h-index via counting sort: bucket citations capped at N, scan from high to low
// accumulating papers until count >= citation level. Time O(N), Space O(N).
"use strict";

function hIndex(citations) {
    const n = citations.length;
    const bucket = new Array(n + 1).fill(0);
    for (const c of citations) bucket[Math.min(c, n)]++;
    let acc = 0;
    for (let h = n; h >= 0; --h) {
        acc += bucket[h];
        if (acc >= h) return h;
    }
    return 0;
}

const citations = [4, 3, 0, 1, 5];
console.log(hIndex(citations));
