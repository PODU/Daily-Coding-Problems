// Boat rescue: min boats, <=2 people each, weight limit k.
// Greedy two-pointer: sort, pair lightest with heaviest if sum<=k. O(n log n) time, O(1) extra.
function numRescueBoats(w, k) {
  w = [...w].sort((a, b) => a - b);
  let lo = 0, hi = w.length - 1, boats = 0;
  while (lo <= hi) {
    if (w[lo] + w[hi] <= k) lo++;
    hi--;
    boats++;
  }
  return boats;
}

console.log(numRescueBoats([100, 200, 150, 80], 200)); // expected 3
