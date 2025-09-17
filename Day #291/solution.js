// Boats: <=2 people, weight limit k, minimize boats. Sort + two pointers.
// Pair lightest with heaviest if they fit, else heaviest alone. Time O(n log n), Space O(1).
function numBoats(weights, k) {
  const w = [...weights].sort((a, b) => a - b);
  let l = 0, h = w.length - 1, boats = 0;
  while (l <= h) {
    if (w[l] + w[h] <= k) l++;
    h--;
    boats++;
  }
  return boats;
}

console.log(numBoats([100, 200, 150, 80], 200));
