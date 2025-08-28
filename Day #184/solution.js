// Day 184: GCD of n numbers via iterated Euclidean algorithm.
// Time O(n * log(max)), Space O(1).
function gcd2(a, b) {
  while (b) {
    [a, b] = [b, a % b];
  }
  return a;
}

function gcdAll(nums) {
  return nums.reduce((g, x) => gcd2(g, x), 0);
}

console.log(gcdAll([42, 56, 14]));
