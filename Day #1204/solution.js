// Day 1204: GCD of n numbers.
// Fold Euclidean gcd across the list. Time O(n log max), Space O(1).
function gcd(a, b) { return b === 0 ? a : gcd(b, a % b); }

function gcdAll(nums) {
  return nums.reduce((g, x) => gcd(g, x), 0);
}

console.log(gcdAll([42, 56, 14])); // 14
