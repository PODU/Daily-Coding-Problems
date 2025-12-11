// Day 732: Minimum boats (each holds <=2 people, weight limit k).
// Approach: Sort; two pointers pair lightest with heaviest when they fit.
// Time: O(n log n), Space: O(1).

function numBoats(weights, k) {
  weights = [...weights].sort((a, b) => a - b);
  let i = 0, j = weights.length - 1, boats = 0;
  while (i <= j) {
    if (weights[i] + weights[j] <= k) i++;
    j--;
    boats++;
  }
  return boats;
}

console.log(numBoats([100, 200, 150, 80], 200)); // 3
