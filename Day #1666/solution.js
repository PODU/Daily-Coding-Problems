// Day 1666: GCD of n numbers.
// Approach: fold Euclid's algorithm across the list. Time O(n*log(max)), Space O(1).
function gcd2(a, b) {
  a = Math.abs(a); b = Math.abs(b);
  while (b) { [a, b] = [b, a % b]; }
  return a;
}

function gcdList(nums) {
  return nums.reduce((g, x) => gcd2(g, x), 0);
}

console.log(gcdList([42, 56, 14])); // 14
