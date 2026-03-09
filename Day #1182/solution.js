// Day 1182: Split N into k contiguous partitions minimizing the maximum sum.
// Binary search the answer in [max element, total]; greedy feasibility check.
// Time O(N log(sum)), Space O(1).

function splitArray(a, k) {
    const feasible = (cap) => {
        let cur = 0, parts = 1;
        for (const x of a) {
            if (cur + x > cap) { parts++; cur = x; if (parts > k) return false; }
            else cur += x;
        }
        return true;
    };
    let lo = Math.max(...a);
    let hi = a.reduce((s, x) => s + x, 0);
    while (lo < hi) {
        const mid = Math.floor((lo + hi) / 2);
        if (feasible(mid)) hi = mid;
        else lo = mid + 1;
    }
    return lo;
}

console.log(splitArray([5, 1, 2, 7, 3, 4], 3)); // 8
