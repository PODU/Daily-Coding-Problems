// Largest product of three: max(max1*max2*max3, min1*min2*max1) tracking top-3 & bottom-2. Time O(n), Space O(1).
function largestTripleProduct(nums) {
  let max1 = -Infinity, max2 = -Infinity, max3 = -Infinity;
  let min1 = Infinity, min2 = Infinity;
  for (const x of nums) {
    if (x > max1) { max3 = max2; max2 = max1; max1 = x; }
    else if (x > max2) { max3 = max2; max2 = x; }
    else if (x > max3) { max3 = x; }
    if (x < min1) { min2 = min1; min1 = x; }
    else if (x < min2) { min2 = x; }
  }
  return Math.max(max1 * max2 * max3, min1 * min2 * max1);
}

console.log(largestTripleProduct([-10, -10, 5, 2]));
