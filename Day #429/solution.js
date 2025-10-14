// Day 429: kth row of Pascal's triangle (0-indexed: row 0 = [1]).
// Multiplicative recurrence row[j] = row[j-1]*(k-j+1)/j. Time O(k), Space O(k).
function pascalRow(k) {
  const row = new Array(k + 1);
  row[0] = 1;
  for (let j = 1; j <= k; j++) row[j] = (row[j - 1] * (k - j + 1)) / j;
  return row;
}

console.log(pascalRow(4).join(" "));
