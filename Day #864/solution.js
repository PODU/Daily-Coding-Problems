// Day 864: Minimum rescue boats (<=2 people, total weight <= k).
// Approach: sort, greedily pair lightest with heaviest using two pointers.
// Time: O(n log n), Space: O(1).
'use strict';

function numRescueBoats(weights, k) {
  weights.sort((a, b) => a - b);
  let i = 0, j = weights.length - 1, boats = 0;
  while (i <= j) {
    if (weights[i] + weights[j] <= k) i++;
    j--;
    boats++;
  }
  return boats;
}

console.log(numRescueBoats([100, 200, 150, 80], 200)); // 3
