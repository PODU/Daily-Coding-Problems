// H-index via counting buckets: bucket papers by min(citation, n), scan from high. O(n) time, O(n) space.
"use strict";

function hIndex(citations) {
    const n = citations.length;
    const bucket = new Array(n + 1).fill(0);
    for (const c of citations) bucket[Math.min(c, n)]++;
    let count = 0;
    for (let h = n; h >= 0; h--) {
        count += bucket[h];
        if (count >= h) return h;
    }
    return 0;
}

function main() {
    const citations = [4, 3, 0, 1, 5];
    console.log(hIndex(citations));
}

main();
