// Largest product of three: track top-3 max and bottom-2 min in one pass.
// Answer = max(max1*max2*max3, min1*min2*max1). O(n) time, O(1) space.

function largestProductOfThree(a) {
  let max1 = -Infinity, max2 = -Infinity, max3 = -Infinity;
  let min1 = Infinity, min2 = Infinity;
  for (const x of a) {
    if (x > max1) { max3 = max2; max2 = max1; max1 = x; }
    else if (x > max2) { max3 = max2; max2 = x; }
    else if (x > max3) { max3 = x; }
    if (x < min1) { min2 = min1; min1 = x; }
    else if (x < min2) { min2 = x; }
  }
  return Math.max(max1 * max2 * max3, min1 * min2 * max1);
}

console.log(largestProductOfThree([-10, -10, 5, 2])); // 500
