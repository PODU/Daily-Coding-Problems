// Boats: sort, greedily pair lightest+heaviest (two-pointer). Time O(n log n).
function numBoats(weights, k) {
  const w = [...weights].sort((a, b) => a - b);
  let i = 0, j = w.length - 1, boats = 0;
  while (i <= j) {
    if (w[i] + w[j] <= k) i++;
    j--;
    boats++;
  }
  return boats;
}

console.log(numBoats([100, 200, 150, 80], 200));
