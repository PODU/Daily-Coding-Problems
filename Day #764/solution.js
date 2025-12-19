// Day 764: Arrange numbers to form the largest integer.
// Sort by comparator: a+b vs b+a (descending). Time: O(n log n * L), Space: O(n).
"use strict";

function largestNumber(nums) {
    const s = nums.map(String);
    s.sort((a, b) => (b + a).localeCompare(a + b));
    if (s[0] === "0") return "0";
    return s.join("");
}

console.log(largestNumber([10, 7, 76, 415]));   // 77641510
