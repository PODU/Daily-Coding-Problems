// Day 1266: Arrange numbers to form the largest integer.
// Sort by custom comparator a+b vs b+a (descending). O(n log n) comparisons.
function largestNumber(nums) {
  const s = nums.map(String);
  s.sort((a, b) => (b + a).localeCompare(a + b));
  if (s.length === 0 || s[0] === "0") return "0";
  return s.join("");
}

console.log(largestNumber([10, 7, 76, 415]));
