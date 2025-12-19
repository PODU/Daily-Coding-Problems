// Day 763: Sliding window maximum via a monotonic decreasing deque of indices.
// Time: O(n), Space: O(k). Prints each window max as it is computed.
"use strict";

function slidingMax(a, k) {
    const dq = [];            // indices, values decreasing
    let head = 0;
    const out = [];
    for (let i = 0; i < a.length; i++) {
        while (dq.length > head && a[dq[dq.length - 1]] <= a[i]) dq.pop();
        dq.push(i);
        if (dq[head] <= i - k) head++;
        if (i >= k - 1) out.push(a[dq[head]]);
    }
    console.log("[" + out.join(", ") + "]");
}

slidingMax([10, 5, 2, 7, 8, 7], 3);   // [10, 7, 8, 8]
