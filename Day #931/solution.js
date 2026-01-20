// Day 931: GCD of n numbers by folding pairwise gcd.
// Time: O(n * log(maxVal)), Space: O(1).
function gcd(a, b) {
  while (b !== 0) { const t = a % b; a = b; b = t; }
  return a;
}
function gcdList(nums) {
  return nums.reduce((g, x) => gcd(g, x), 0);
}

console.log(gcdList([42, 56, 14])); // 14
