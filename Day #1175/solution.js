// Day 1175: Maximum subarray sum in a circular array (empty allowed -> 0).
// Answer = max(0, kadaneMax, total - kadaneMin); total-min covers the wrap case.
// Time O(N), Space O(1).

function maxCircularSubarray(a) {
    let total = 0, curMax = 0, bestMax = 0, curMin = 0, bestMin = 0;
    for (const x of a) {
        total += x;
        curMax = Math.max(x, curMax + x); bestMax = Math.max(bestMax, curMax);
        curMin = Math.min(x, curMin + x); bestMin = Math.min(bestMin, curMin);
    }
    return Math.max(0, bestMax, total - bestMin);
}

console.log(maxCircularSubarray([8, -1, 3, 4])); // 15
console.log(maxCircularSubarray([-4, 5, 1, 0])); // 6
