// Day 1731: kth row of Pascal's triangle (1-indexed) using O(k) space.
// Binomial coefficients built in place. Time O(k), Space O(k).
function pascalRow(k) {
  const row = new Array(k).fill(1);
  for (let i = 1; i < k; i++) row[i] = (row[i - 1] * (k - i)) / i;
  return row;
}

const k = 5; // row [1,4,6,4,1]
console.log(pascalRow(k).join(" "));
