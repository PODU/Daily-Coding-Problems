// Day 1472: Largest product of any three integers.
// Track 3 largest and 2 smallest in one pass; max of two candidate products.
// Time O(N), Space O(1).

function maxProductThree(nums) {
  let max1 = -Infinity, max2 = -Infinity, max3 = -Infinity;
  let min1 = Infinity, min2 = Infinity;
  for (const n of nums) {
    if (n > max1) { max3 = max2; max2 = max1; max1 = n; }
    else if (n > max2) { max3 = max2; max2 = n; }
    else if (n > max3) { max3 = n; }
    if (n < min1) { min2 = min1; min1 = n; }
    else if (n < min2) { min2 = n; }
  }
  return Math.max(max1 * max2 * max3, max1 * min1 * min2);
}

console.log(maxProductThree([-10, -10, 5, 2])); // 500
