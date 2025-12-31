// Day 830: Largest number formed by concatenating the given numbers.
// Sort strings by comparator (a+b) vs (b+a) descending. O(N log N) compares of O(L) strings.
"use strict";

function largestNumber(nums) {
  const strs = nums.map(String);
  strs.sort((a, b) => (b + a < a + b ? -1 : b + a > a + b ? 1 : 0));
  const result = strs.join("");
  if (result.length && result[0] === "0") return "0";
  return result;
}

function main() {
  console.log(largestNumber([10, 7, 76, 415])); // 77641510
}

main();
