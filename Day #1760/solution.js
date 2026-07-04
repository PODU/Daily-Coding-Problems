// Day 1760: Sliding window maximum.
// Approach: monotonic deque of indices (decreasing values). O(n) time, O(k) space.

function maxSlidingWindow(a, k) {
    const res = [];
    const dq = []; // indices, values decreasing (head at index 0)
    let head = 0;
    for (let i = 0; i < a.length; i++) {
        if (head < dq.length && dq[head] <= i - k) head++;
        while (head < dq.length && a[dq[dq.length - 1]] <= a[i]) dq.pop();
        dq.push(i);
        if (i >= k - 1) res.push(a[dq[head]]);
    }
    return res;
}

function main() {
    const a = [10, 5, 2, 7, 8, 7];
    const k = 3;
    const res = maxSlidingWindow(a, k);
    console.log("[" + res.join(", ") + "]");
}

main();
