// Day 1181: Find the minimum in a rotated sorted array (no duplicates).
// Binary search: compare mid to the right end to discard the sorted half.
// Time O(log N), Space O(1).

function findMin(a) {
    let lo = 0, hi = a.length - 1;
    while (lo < hi) {
        const mid = (lo + hi) >> 1;
        if (a[mid] > a[hi]) lo = mid + 1;
        else hi = mid;
    }
    return a[lo];
}

console.log(findMin([5, 7, 10, 3, 4])); // 3
