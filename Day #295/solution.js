// Kth row of Pascal's triangle (1-indexed) via iterative binomials in one array. O(k) space, O(k) time.
function pascalRow(k) {
  const n = k - 1; // 0-indexed row number
  const row = new Array(k).fill(1);
  for (let r = 1; r < k; r++) row[r] = (row[r-1] * (n - r + 1)) / r;
  return row;
}

console.log("[" + pascalRow(5).join(", ") + "]");
