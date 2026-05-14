// Sort number-strings by comparator a+b > b+a (largest concat first), join; handle all-zeros.
// Time: O(n log n * L) comparisons, Space: O(n).
function largestNumber(nums) {
  const s = nums.map(String);
  s.sort((a, b) => (b + a > a + b ? 1 : b + a < a + b ? -1 : 0));
  if (s[0] === "0") return "0";
  return s.join("");
}

if (require.main === module) {
  console.log(largestNumber([10, 7, 76, 415]));
}
